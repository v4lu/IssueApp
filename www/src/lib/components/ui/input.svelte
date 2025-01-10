<script lang="ts">
	import type { VariantProps } from 'class-variance-authority';
	import type { HTMLInputAttributes } from 'svelte/elements';
	import { inputVariants } from '.';
	import { cn } from '$lib';

	type Variant = VariantProps<typeof inputVariants>['variant'];

	type InputProps = HTMLInputAttributes & {
		class?: string;
		value?: string;
		variant?: Variant;
	};

	let { variant, class: className, value = $bindable(), ...rest }: InputProps = $props();
</script>

<input bind:value class={cn(inputVariants({ variant, className }))} {...rest} />
