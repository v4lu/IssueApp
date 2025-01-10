<script lang="ts">
	import Icon from '@iconify/svelte';
	import { DefaultWrapper, SettingsWrapper } from '$lib/components/layout';
	import { Avatar } from '$lib/components/ui/avatar';
	import { Button } from '$lib/components/ui/button';
	import { useOrgMembers } from '$lib/api/org-members.svelte.js';
	import { InviteUserOrg } from '$lib/components/modals';
	import { cn } from '$lib';
	import { Dropdown } from '$lib/components/ui/dropdown/index.js';
	import type { MemberRole } from '$lib/types/org.type.js';

	let { data } = $props();
	const { resp, updateRole, removeMember, addMember } = useOrgMembers(
		data.accessToken,
		data.org_id
	);

	let isOpenInviteModal = $state(false);
	let isDropdownRoleOpen = $state(false);
	let roles = $state<MemberRole[]>(['ADMIN', 'MEMBER', 'OWNER']);

	function getMemberColor(role: string) {
		switch (role) {
			case 'OWNER':
				return 'bg-red-100 text-red-600';
			case 'ADMIN':
				return 'bg-yellow-100 text-yellow-600';
			case 'MEMBER':
				return 'bg-green-100 text-green-600';
			default:
				return 'bg-gray-100 text-gray-600';
		}
	}
</script>

<DefaultWrapper>
	<SettingsWrapper class="flex flex-col gap-6 " title="Team Members">
		<div class="flex w-full items-center justify-end">
			<Button onclick={() => (isOpenInviteModal = true)}>
				<Icon icon="ph:plus" class="mr-2 h-4 w-4" />
				Invite Member
			</Button>
		</div>

		<div class="rounded-md border">
			{#if resp.isLoading}
				<p>loading...</p>
			{:else}
				<table class="w-full">
					<thead class="w-full">
						<tr class="border-b bg-muted">
							<th class="p-4 text-left text-sm font-medium">Member</th>
							<th class="p-4 text-left text-sm font-medium">Role</th>
							{#if resp.members[0].org_member.role === 'OWNER' || resp.members[0].org_member.role === 'ADMIN'}
								<th class="p-4 text-right text-sm font-medium">Actions</th>
							{/if}
						</tr>
					</thead>
					<tbody>
						{#each resp.members as { org_member, user }}
							<tr class="border-b last:border-0">
								<td class="p-4">
									<div class="flex items-center gap-3">
										<Avatar {user} />
										<div>
											<div class="font-medium">{user.username}</div>
											<div class="text-sm text-muted-foreground">{user.email}</div>
										</div>
									</div>
								</td>
								<td class="p-4">
									<div
										class={cn(
											'inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ',

											getMemberColor(org_member.role)
										)}
									>
										{org_member.role}
									</div>
								</td>

								{#if data.session_role === 'OWNER' || data.session_role === 'ADMIN'}
									<td class="p-4 text-right">
										{#if org_member.role !== 'OWNER'}
											<div class="flex justify-end gap-2">
												<Dropdown
													triggerText="Change Role"
													triggerClass="min-w-0"
													isOpen={isDropdownRoleOpen}
												>
													<ul class="grid gap-2">
														{#each roles as role}
															<Button
																variant={role === org_member.role ? 'secondary' : 'ghost'}
																size="sm"
																onclick={() => {
																	updateRole(org_member.user_id, role);
																	isDropdownRoleOpen = false;
																}}
															>
																{role}
															</Button>
														{/each}
													</ul>
												</Dropdown>
												<Button
													onclick={() => removeMember(org_member.user_id)}
													variant="destructive"
													size="sm"
												>
													<Icon icon="ph:trash" class="mr-2 h-4 w-4" />
													Remove
												</Button>
											</div>
										{/if}
									</td>
								{/if}
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	</SettingsWrapper>
</DefaultWrapper>

<InviteUserOrg bind:isOpen={isOpenInviteModal} isSubmitting={resp.isInviting} inviteUser={addMember} />
