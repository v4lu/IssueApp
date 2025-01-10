<script lang="ts">
	import Icon from '@iconify/svelte';
	import { CardWrapper, DefaultWrapper, SettingsWrapper } from '$lib/components/layout';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Modal } from '$lib/components/ui/modal';
	import { Tooltip } from '$lib/components/ui/tooltip/index.js';
	import { useOrg } from '$lib/api/org.svelte.js';
	import { orgStore, orgsStore } from '$lib/stores/org.store.js';
	import { toast } from '$lib/stores/toast.store.js';
	import { browser } from '$app/environment';
	import { DeleteModal } from '$lib/components/modals/index.js';

	let { data } = $props();
	const { updateOrg, deleteOrg } = useOrg(data.accessToken);

	let org_name = $state($orgsStore.find((org) => org.id === data.org.id)?.name);
	let org_logo = $state($orgsStore.find((org) => org.id === data.org.id)?.logo_url ?? null);
	let isImageModalOpen = $state(false);
	let isDeleteModalOpen = $state(false);

	function handleImageDelete() {
		org_logo = null;
	}

	function handleImageUpload() {
		console.log('Upload image');
	}

	async function handleUpdate() {
		if (!org_name) return;
		const response = await updateOrg(data.org.id, org_name, org_logo);
		if (response) {
			orgsStore.updateOrg(data.org.id, { ...response });

			if (response.id === $orgStore.id) {
				orgStore.updateOrg({ ...response });
			}

			org_logo = response.logo_url;
			org_name = response.name;
		}

		toast.success('Organization updated successfully');
	}

	async function handleDeleteOrganization() {
		if (data.orgs_count === 1) {
			toast.error('You cannot delete the last organization');
			return;
		}

		try {
			await deleteOrg(data.org.id);
			orgsStore.deleteOrg(data.org.id);
			if (data.org_id === $orgStore.id) {
				if (browser) {
					toast.success('Organization deleted successfully');

					setTimeout(() => {
						window.history.replaceState(null, '', '/');
						window.location.replace('/');
						window.location.reload();
					}, 500);
					return;
				}
			}
			toast.success('Organization deleted successfully');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<DefaultWrapper>
	<SettingsWrapper class="grid place-content-start gap-6 pb-6" title={data.org.name}>
		<CardWrapper class="p-6">
			<div class="mt-4 space-y-8">
				<div class="flex flex-col gap-12 md:flex-row md:items-start">
					<div class="space-y-3">
						<p class="text-sm font-medium text-foreground/80">Organization Logo</p>
						<div class="group relative">
							{#if org_logo}
								<div class="relative">
									<img
										src={org_logo}
										alt="Organization logo"
										class="h-32 w-32 rounded-lg border border-border object-cover transition-all hover:opacity-90"
									/>
									<div
										class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/50 opacity-0 transition-opacity group-hover:opacity-100"
									>
										<div class="flex gap-2">
											<Button
												variant="secondary"
												size="sm"
												class="!p-2"
												onclick={() => (isImageModalOpen = true)}
											>
												<Icon icon="solar:pen-2-linear" class="size-4" />
											</Button>
											<Button
												variant="destructive"
												size="sm"
												class="!p-2"
												onclick={handleImageDelete}
											>
												<Icon icon="solar:trash-bin-trash-linear" class="size-4" />
											</Button>
										</div>
									</div>
								</div>
							{:else}
								<button
									class="flex h-32 w-32 flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-border bg-muted/50 transition-colors hover:bg-muted"
									onclick={() => (isImageModalOpen = true)}
								>
									<Icon icon="solar:gallery-wide-linear" class="size-8 text-muted-foreground" />
									<span class="text-xs text-muted-foreground">Upload Logo</span>
								</button>
							{/if}
						</div>
					</div>

					<div class="flex-1 space-y-3">
						<p class="text-sm font-medium text-foreground/80">Organization Details</p>
						<div class="space-y-2">
							<label for="org-name" class="text-sm text-muted-foreground">Organization Name</label>
							<Input
								id="org-name"
								bind:value={org_name}
								class="max-w-md"
								placeholder="Enter organization name"
							/>
						</div>
					</div>
				</div>

				<div class="flex justify-end">
					<Button onclick={handleUpdate} class="min-w-[100px]">
						<Icon icon="solar:disk-linear" class="mr-2 size-4" />
						Update
					</Button>
				</div>
			</div>
		</CardWrapper>

		<a href={`/settings/org/${data.org.id}/members`}>
			<CardWrapper
				class="
            group
            p-6 hover:bg-primary/5"
			>
				<div class="flex w-full items-center justify-between">
					<div class="flex items-center gap-6">
						<Icon icon="solar:users-group-two-rounded-line-duotone" class="size-9" />
						<div>
							<h3 class="text-lg font-medium">Members</h3>
							<p class="text-sm text-muted-foreground">Manage organization members and roles.</p>
						</div>
					</div>
					<Icon
						icon="lucide:chevron-right"
						class="size-6
                    transition-transform duration-300 ease-in-out
                    group-hover:translate-x-2 group-hover:transform
                    "
					/>
				</div>
			</CardWrapper>
		</a>

		<h3 class="text-lg font-medium">Danger Zone</h3>

		<CardWrapper class="p-6">
			<div class="rounded-lg border border-destructive/20 bg-destructive/5">
				<div class="p-4">
					<div class="flex items-center justify-between gap-6">
						<div class="space-y-1">
							<p class="text-sm font-medium">Leave Organization</p>
							<p class="text-balance text-sm text-muted-foreground">
								Remove yourself from this organization. You will lose access to all resources.
							</p>
						</div>
						{#if data.orgs_count === 1}
							<Tooltip
								variant="destructive"
								content="You cannot leave the last organization"
								position="top"
							>
								<Button
									variant="destructive"
									size="sm"
									disabled
									onclick={() => console.log('Leave organization')}
									class="shrink-0"
								>
									<Icon icon="solar:logout-2-linear" class="mr-2 size-4" />
									Leave
								</Button>
							</Tooltip>
						{:else}
							<Button
								variant="destructive"
								size="sm"
								onclick={() => console.log('Leave organization')}
								class="shrink-0"
							>
								<Icon icon="solar:logout-2-linear" class="mr-2 size-4" />
								Leave
							</Button>
						{/if}
					</div>
				</div>
			</div>

			<div class="mt-4 space-y-4">
				<div class="rounded-lg border border-destructive/20 bg-destructive/5">
					<div class="p-4">
						<div class="flex items-center justify-between gap-6">
							<div class="space-y-1">
								<p class="text-sm font-medium">Delete Organization</p>
								<p class="text-balance text-sm text-muted-foreground">
									Permanently delete your organization and all of its data. This action cannot be
									undone.
								</p>
							</div>

							{#if data.orgs_count === 1}
								<Tooltip
									content="You cannot delete the last organization"
									variant="destructive"
									position="top"
								>
									<Button variant="destructive" size="sm" disabled class="shrink-0">
										<Icon icon="solar:trash-bin-trash-linear" class="mr-2 size-4" />
										Delete
									</Button>
								</Tooltip>
							{:else}
								<Button
									variant="destructive"
									size="sm"
									onclick={() => (isDeleteModalOpen = true)}
									class="shrink-0"
								>
									<Icon icon="solar:trash-bin-trash-linear" class="mr-2 size-4" />
									Delete
								</Button>
							{/if}
						</div>
					</div>
				</div>
			</div>
		</CardWrapper>
	</SettingsWrapper>
</DefaultWrapper>

<Modal bind:isOpen={isImageModalOpen} onClose={() => (isImageModalOpen = false)}>
	<div class="space-y-4">
		<div class="flex justify-end gap-2">
			<Button variant="outline" onclick={() => (isImageModalOpen = false)}>Cancel</Button>
			<Button onclick={handleImageUpload}>Upload</Button>
		</div>
	</div>
</Modal>

<DeleteModal
	bind:isOpen={isDeleteModalOpen}
	onConfirm={handleDeleteOrganization}
	title="Delete Organization"
	description="Permanently delete your organization and all of its data. This action cannot be undone."
/>
