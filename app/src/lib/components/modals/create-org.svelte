<script lang="ts">
	import { Button } from '../ui/button';
	import { Field } from '../ui/field';
	import { Input } from '../ui/input';
	import { Modal } from '../ui/modal';
	import { orgStore, orgsStore } from '$lib/stores/org.store';
	import { useOrg } from '$lib/api/org.svelte';

	type Props = {
		isOpen: boolean;
		authToken: string;
	};
	let { isOpen = $bindable(), authToken }: Props = $props();
	const { resp, createOrg } = useOrg(authToken);

	let name = $state('');

	async function handleCreateWorkspace() {
		const org = await createOrg(name);
		if (org) {
			orgStore.setOrg(org);
			orgsStore.addOrg(org);
		}
		name = '';
		isOpen = false;
	}
</script>

<Modal onClose={() => (isOpen = false)} bind:isOpen class="md:w-[35rem]">
	<div class="mt-3 grid flex-col gap-3 p-8 pt-3">
		<Field name="Name">
			<Input type="text" bind:value={name} placeholder="Scyllaflow" />
		</Field>

		<div class="flex justify-end">
			<Button
				onclick={handleCreateWorkspace}
				disabled={resp.isCreatingOrg}
				isLoading={resp.isCreatingOrg}
				class="w-fit"
			>
				Create
			</Button>
		</div>
	</div>
</Modal>
