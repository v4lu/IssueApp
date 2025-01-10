<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { VariantProps } from 'class-variance-authority';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { buttonVariants } from '.';
	import { cn } from '$lib';

	type Variant = VariantProps<typeof buttonVariants>['variant'];
	type Size = VariantProps<typeof buttonVariants>['size'];

	type ButtonPropsType = HTMLButtonAttributes & {
		isLoading?: boolean;
		class?: string;
		variant?: Variant;
		size?: Size;
	};

	let {
		isLoading,
		class: className,
		variant = 'default',
		size = 'default',
		children,
		...rest
	}: ButtonPropsType = $props();
</script>

<button {...rest} class={cn(buttonVariants({ variant, size, className }))}>
	{#if isLoading}
		<Icon icon="eos-icons:loading" class="mr-2 size-4 animate-spin" />
	{/if}
	{#if children}
		{@render children()}
	{/if}
</button>
