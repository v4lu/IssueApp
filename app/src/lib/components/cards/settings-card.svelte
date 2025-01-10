<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		icon: string;
		title: string;
		description: string;
		children: Snippet;
	};

	let { icon, title, description, children }: Props = $props();
</script>

<div class="flex w-full items-center justify-between">
	<div class="flex items-center gap-6">
		<Icon {icon} class="size-7" />
		<div>
			<h3 class="text-lg font-medium">{title}</h3>
			<p class="text-sm text-muted-foreground">{description}</p>
		</div>
	</div>
	{@render children()}
</div>
