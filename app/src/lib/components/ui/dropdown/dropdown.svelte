<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { Component, Snippet } from 'svelte';
	import { cubicOut } from 'svelte/easing';
	import { fade, slide } from 'svelte/transition';
	import type { IconProps } from '$lib/components/icons';
	import { cn } from '$lib';

	type Props = {
		isOpen: boolean;
		class?: string;
		children: Snippet;
		triggerText?: string;
		triggerIcon?: string;
		triggerIconClass?: string;
		triggerClass?: string;
		downArrowIcon?: boolean;
		CustomIcon?: Component<IconProps>;
		customIconClass?: string;
		triggerIconPosition?: 'left' | 'right';
		wrapperClass?: string;
	};

	let {
		triggerText,
		triggerIcon,
		triggerClass,
		triggerIconClass,
		isOpen = $bindable(),
		class: className,
		downArrowIcon = false,
		CustomIcon,
		customIconClass,
		triggerIconPosition = 'right',
		wrapperClass,
		children
	}: Props = $props();

	let btn: HTMLButtonElement;

	function clickOutside(node: HTMLElement) {
		function handleClick(event: MouseEvent) {
			if (btn && btn.contains(event?.target as Node)) return;
			if (node && !node.contains(event.target as Node)) {
				isOpen = false;
			}
		}
		document.addEventListener('click', handleClick, true);
		return {
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	}
</script>

<div class={cn('relative', wrapperClass)}>
	<button
		bind:this={btn}
		type="button"
		onclick={() => (isOpen = !isOpen)}
		class={cn(
			'flex min-w-[12rem] items-center gap-3 rounded-md border border-border py-2 pl-4 pr-3 text-xs shadow-sm',
			triggerClass
		)}
	>
		{#if CustomIcon}
			<CustomIcon class={customIconClass} />
		{/if}
		{#if !downArrowIcon && triggerIcon && triggerIconPosition === 'left'}
			<Icon icon={triggerIcon} class={cn('ml-auto size-4', triggerIconClass)} />
		{/if}
		{#if triggerText}
			<span>{triggerText}</span>
		{/if}
		{#if downArrowIcon}
			<Icon
				icon="solar:alt-arrow-down-outline"
				class={cn(
					'ml-auto size-4',
					triggerIconClass,
					isOpen
						? 'rotate-180 transition-transform duration-200 ease-in'
						: 'rotate-0 transition-transform duration-200 ease-out'
				)}
			/>
		{/if}
		{#if !downArrowIcon && triggerIcon && triggerIconPosition === 'right'}
			<Icon icon={triggerIcon} class={cn('ml-auto size-4', triggerIconClass)} />
		{/if}
	</button>

	{#if isOpen}
		<div
			use:clickOutside
			transition:slide={{ duration: 300, easing: cubicOut }}
			class={cn(
				'absolute top-[2.8rem] z-[120] w-full min-w-[12rem] rounded-md border border-border bg-card shadow-md',
				className
			)}
		>
			<div transition:fade={{ duration: 200 }} class="p-2">
				{@render children()}
			</div>
		</div>
	{/if}
</div>
