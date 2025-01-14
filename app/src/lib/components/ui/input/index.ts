import { type VariantProps, cva } from 'class-variance-authority';

export { default as Input } from './input.svelte';

export const inputVariants = cva('', {
	variants: {
		variant: {
			default:
				'border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm shadow-sm file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50',
			empty: 'mr-3 w-full appearance-none border-none bg-transparent px-2 py-1 focus:outline-none'
		}
	},
	defaultVariants: {
		variant: 'default'
	}
});

export type InputVariants = VariantProps<typeof inputVariants>;
