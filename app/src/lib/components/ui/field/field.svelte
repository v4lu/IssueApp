<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib';

	type FieldProps = {
		name: string;
		description?: string;
		optional?: boolean;
		error?: string[] | string;
		class?: string;
		children: Snippet;
	};

	let { name, description, optional, error, children, class: className }: FieldProps = $props();

	const forDescription = name.toLowerCase().replace(/ /g, '-');
</script>

<div class={cn('grid gap-2', className)}>
	<div>
		<label class="flex items-center justify-start gap-0.5 text-sm" for={forDescription}
			>{name}
			{#if optional}
				<span class="text-xs text-muted-foreground">(optional)</span>
			{/if}
		</label>
		{#if description}
			<p class="text-xs text-muted-foreground">{description}</p>
		{/if}
	</div>
	{@render children()}
	{#if error}
		<p class="text-xs text-destructive">{error}</p>
	{/if}
</div>
