<script lang="ts">
	import type { Snippet } from 'svelte';
	import { backOut, quintOut } from 'svelte/easing';
	import { fade, fly } from 'svelte/transition';
	import { cn } from '$lib';

	type ModalProps = {
		onClose: () => void;
		class?: string;
		children: Snippet;
		isOpen: boolean;
	};

	let { onClose, children, class: className, isOpen = $bindable() }: ModalProps = $props();
	let modalContent = $state<HTMLDivElement>();

	function handleOutsideClick(event: MouseEvent) {
		if (modalContent && !modalContent.contains(event.target as Node)) {
			onClose();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') {
			onClose();
		}
	}

	$effect(() => {
		document.addEventListener('mousedown', handleOutsideClick);
		document.addEventListener('keydown', handleKeydown);
		return () => {
			document.removeEventListener('mousedown', handleOutsideClick);
			document.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-[100] flex items-end justify-center bg-black/70 backdrop-blur-sm md:items-center md:p-12"
		transition:fade={{ duration: 200 }}
	>
		<div
			bind:this={modalContent}
			class={cn(
				'w-full md:w-fit',
				'relative rounded-t-3xl border border-border bg-card p-6  md:rounded-md',
				'shadow-[0_40px_45px_-5px_rgba(0,0,0,0.1),0_10px_10px_-5px_rgba(0,0,0,0.04)]',
				className
			)}
			in:fly={{ y: -50, duration: 300, easing: backOut }}
			out:fly={{ y: -20, duration: 200, easing: quintOut }}
		>
			{@render children()}
		</div>
	</div>
{/if}
