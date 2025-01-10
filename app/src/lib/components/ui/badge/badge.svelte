<script lang="ts">
	import { cn } from '$lib';
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
		class?: string;
	};

	let { children, class: className }: Props = $props();
</script>

<span
	class={cn(
		'inline-flex items-center rounded-md bg-primary/10 px-2 py-1 text-xs font-medium text-primary ring-1 ring-inset ring-primary/20',
		className
	)}
>
	{@render children()}
</span>
