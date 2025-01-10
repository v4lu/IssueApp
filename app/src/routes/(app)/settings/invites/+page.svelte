<script lang="ts">
	import { useInvites } from '$lib/api/invites.svelte';
	import { DefaultWrapper, SettingsWrapper } from '$lib/components/layout';
	import { Button } from '$lib/components/ui/button';
	let { data } = $props();
	const { resp, acceptInvite, cancelInvite } = useInvites(data.accessToken);
</script>

<DefaultWrapper>
	<SettingsWrapper class="flex flex-col gap-6 pb-6" title="Invites">
		{#if resp.isLoading}
			<div class="flex h-64 items-center justify-center">loading</div>
		{:else if resp.invites.length === 0}
			<div class="flex h-64 items-center justify-center">
				<p class="text-gray-500">No invites found</p>
			</div>
		{:else}
			{#each resp.invites as { org_member_invite, org_logo, org_name, invited_by }}
				<div class="flex items-center justify-between p-4">
					<div class="flex items-center space-x-4">
						{#if org_logo}
							<img src={org_logo} alt={org_name} class="h-12 w-12 rounded-full" />
						{:else}
							<div class="h-12 w-12 rounded-full bg-muted"></div>
						{/if}
						<div>
							<p class="text-lg font-semibold">{org_name}</p>
							<p class="text-sm text-gray-500">Invited by {invited_by.username}</p>
						</div>
					</div>
					<div
						class="items -center
                        flex space-x-4"
					>
						<Button onclick={() => acceptInvite(org_member_invite.token, org_member_invite.org_id)}
							>Accept</Button
						>
						<Button variant="secondary" onclick={() => cancelInvite(org_member_invite.token)}
							>Cancel</Button
						>
					</div>
				</div>
			{/each}
		{/if}
	</SettingsWrapper>
</DefaultWrapper>
