<script lang="ts">
	import { Button } from '../ui/button';
	import { Field } from '../ui/field';
	import { Input } from '../ui/input';
	import { Modal } from '../ui/modal';

	type Props = {
		isOpen: boolean;
		inviteUser: (email: string) => Promise<void>;
		isSubmitting: boolean;
	};
	let { isOpen = $bindable(), inviteUser, isSubmitting }: Props = $props();

	let email = $state('');

	async function handleCreateWorkspace() {
		await inviteUser(email);
		email = '';
		isOpen = false;
	}
</script>

<Modal onClose={() => (isOpen = false)} bind:isOpen class="md:w-[35rem]">
	<div class="mt-3 grid flex-col gap-3 p-8 pt-3">
		<Field name="Name">
			<Input type="text" bind:value={email} placeholder="email@email.com" />
		</Field>

		<div class="flex justify-end">
			<Button onclick={handleCreateWorkspace} disabled={isSubmitting} class="w-fit">Create</Button>
		</div>
	</div>
</Modal>
