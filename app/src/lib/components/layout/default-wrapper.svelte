<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib';

	type Props = {
		children: Snippet;
		class?: string;
	};

	let { children, class: className }: Props = $props();
</script>

<div class="grid px-4 pb-4">
	<main class={cn('border border-border bg-background shadow-sm', className)}>
		{@render children()}
	</main>
</div>
