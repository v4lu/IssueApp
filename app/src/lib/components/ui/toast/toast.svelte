<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	type ToastType = {
		message: string;
		duration?: number;
		type: 'success' | 'error' | 'info';
		close: () => void;
	};
	import { cn } from '$lib';

	let { close, message, type, duration = 1500 }: ToastType = $props();
	let progress = $state(100);

	const iconMap = $state({
		success: 'lucide:circle-check',
		info: 'lucide:circle-alert',
		error: 'lucide:circle-divide'
	});

	const colorMap = $state({
		success: 'bg-green-100 text-green-800',
		error: 'bg-red-100 text-red-800',
		info: 'bg-blue-100 text-blue-800'
	});

	const progressColorMap = $state({
		success: 'bg-green-500',
		error: 'bg-red-500',
		info: 'bg-blue-500'
	});

	onMount(() => {
		const startTime = Date.now();
		const interval = setInterval(() => {
			const elapsed = Date.now() - startTime;
			progress = Math.max(0, 100 - (elapsed / duration) * 100);
			if (progress <= 0) {
				clearInterval(interval);
				close();
			}
		}, 10);
		return () => clearInterval(interval);
	});
</script>

<div
	class={cn('flex min-w-[13rem] flex-col rounded-md p-4 shadow-md', colorMap[type])}
	transition:fade={{ duration: 300 }}
>
	<div class="flex items-start justify-between">
		<div class="flex items-center gap-3">
			<Icon icon={iconMap[type]} class="mt-0.5 size-6 flex-shrink-0" />
			<p class="text-sm">{message}</p>
		</div>
		<button class="ml-4 text-current hover:text-opacity-75 focus:outline-none" onclick={close}>
			<Icon icon="lucide:x" class="size-4" />
		</button>
	</div>
	<div class="mt-2 h-1 w-full rounded-full bg-gray-200/80">
		<div class={cn('h-1 rounded-full', progressColorMap[type])} style="width: {progress}%"></div>
	</div>
</div>
