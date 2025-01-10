<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Modal } from '../ui/modal';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		onConfirm: () => void;
		isOpen: boolean;
		title: string;
		description: string;
	};

	let { isOpen = $bindable(), onConfirm, description, title }: Props = $props();
</script>

<Modal {isOpen} onClose={() => (isOpen = false)} class="p min-w-[30rem]">
	<div class="grid gap-4">
		<h3 class="text-pretty text-left text-2xl font-bold">{title}</h3>
		<p class=" text-left text-muted-foreground">{description}</p>
		<div class="flex w-full items-center justify-end gap-4">
			<Button class="px-8" variant="outline" onclick={() => (isOpen = false)}>Cancel</Button>
			<Button class="px-8" variant="destructive" onclick={onConfirm}>
				<Icon icon="solar:trash-bin-2-bold" class="mr-2 size-5" />
				Delete</Button
			>
		</div>
	</div>
</Modal>
