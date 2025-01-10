<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib';

	type Props = {
		children: Snippet;
		title: string;
		class?: string;
	};

	let { children, title, class: className }: Props = $props();
</script>

<div
	class={cn('scrollbar mx-auto h-[calc(100dvh-83px)] max-w-4xl overflow-y-auto pt-12', className)}
>
	<div class="mb-6">
		<h2 class="text-2xl font-semibold">{title}</h2>
	</div>
	{@render children()}
</div>
