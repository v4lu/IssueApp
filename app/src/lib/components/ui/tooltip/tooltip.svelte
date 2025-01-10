<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fade } from 'svelte/transition';

	type Position = 'top' | 'bottom' | 'left' | 'right';

	type Props = {
		content: string;
		position?: Position;
		children: Snippet;
		variant?: 'default' | 'secondary' | 'destructive';
	};

	let { content, position = 'top', children, variant = 'default' }: Props = $props();

	let show = $state(false);
	let tooltipRef = $state<HTMLDivElement | null>(null);

	const positions = {
		top: 'bottom-full left-1/2 -translate-x-1/2 mb-2',
		bottom: 'top-full left-1/2 -translate-x-1/2 mt-2',
		left: 'right-full top-1/2 -translate-y-1/2 mr-2',
		right: 'left-full top-1/2 -translate-y-1/2 ml-2'
	};

	const arrowPositions = {
		top: 'before:top-full before:left-1/2 before:-translate-x-1/2 before:border-[6px] before:border-transparent before:border-t-current',
		bottom:
			'before:bottom-full before:left-1/2 before:-translate-x-1/2 before:border-[6px] before:border-transparent before:border-b-current',
		left: 'before:left-full before:top-1/2 before:-translate-y-1/2 before:border-[6px] before:border-transparent before:border-l-current',
		right:
			'before:right-full before:top-1/2 before:-translate-y-1/2 before:border-[6px] before:border-transparent before:border-r-current'
	};

	const variantClasses = {
		default: 'bg-popover text-popover-foreground',
		secondary: 'bg-secondary text-secondary-foreground',
		destructive: 'bg-popover'
	};
</script>

<div class="relative inline-block">
	<div onmouseenter={() => (show = true)} onmouseleave={() => (show = false)} class="inline-block">
		{@render children()}
	</div>

	{#if show}
		<div
			bind:this={tooltipRef}
			transition:fade={{ duration: 150 }}
			class="absolute z-50 {positions[position]}
				min-w-max max-w-[15rem]
				rounded-md px-3 py-1.5
				text-xs
				{variantClasses[variant]}
				before:absolute before:content-['']
				{arrowPositions[position]}
				animate-in
				fade-in select-none
				shadow-md
				ring-1 ring-border"
		>
			{content}
		</div>
	{/if}
</div>
