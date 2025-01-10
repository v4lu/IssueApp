<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib';
	type Props = {
		x: number;
		y: number;
		children: Snippet;
		class?: string;
		isOpen: boolean;
		onclick: (e: Event) => void;
	};
	let { x, y, children, class: className, isOpen, onclick }: Props = $props();
	let menuElement = $state<HTMLDivElement>();

	$effect(() => {
		if (menuElement) {
			const viewportWidth = window.innerWidth;
			const viewportHeight = window.innerHeight;
			const menuWidth = menuElement.offsetWidth;
			const menuHeight = menuElement.offsetHeight;

			if (x + menuWidth > viewportWidth) {
				x = viewportWidth - menuWidth;
			}

			y = y - menuHeight / 2;

			if (y < 0) {
				y = 0;
			} else if (y + menuHeight > viewportHeight) {
				y = viewportHeight - menuHeight;
			}
		}
	});
</script>

{#if isOpen}
	<div
		{onclick}
		bind:this={menuElement}
		class={cn('absolute z-[100] rounded-md border border-border bg-card p-2 shadow-md', className)}
		style="left: {x}px; top: {y}px;"
	>
		{@render children()}
	</div>
{/if}
