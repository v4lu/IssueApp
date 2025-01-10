<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib';

	type Props = {
		class?: string;
		children: Snippet;
	};

	let { class: className, children }: Props = $props();
</script>

<div class={cn('rounded-md border border-border p-4', className)}>
	{@render children()}
</div>
