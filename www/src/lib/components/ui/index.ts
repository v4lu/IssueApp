import { type VariantProps, cva } from 'class-variance-authority';

export { default as Button } from './button.svelte';
export { default as Input } from './input.svelte';

export const buttonVariants = cva(
	'inline-flex items-center justify-center whitespace-nowrap rounded-md text-xs font-semibold ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
	{
		variants: {
			variant: {
				default: 'bg-primary text-primary-foreground hover:bg-primary/90',
				outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
				secondary: 'bg-accent text-accent-foreground hover:bg-accent/80',
				ghost: 'hover:bg-accent hover:text-accent-foreground',
				link: 'text-primary underline-offset-4 hover:underline'
			},
			size: {
				default: 'h-9 px-4 py-2',
				xs: 'h-7 rounded px-2 text-xs',
				sm: 'h-8 rounded-md px-3',
				lg: 'h-10 rounded-md px-8',
				icon: 'size-8',
				'icon-sm': 'size-7',
				'icon-xs': 'size-6 p-0'
			}
		},
		defaultVariants: {
			variant: 'default',
			size: 'default'
		}
	}
);

export const inputVariants = cva('', {
	variants: {
		variant: {
			default:
				'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50',
			empty: 'mr-3 w-full appearance-none border-none bg-transparent px-2 py-1 focus:outline-none'
		}
	},
	defaultVariants: {
		variant: 'default'
	}
});
