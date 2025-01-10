<script lang="ts">
	import { type ChainedCommands, Editor } from '@tiptap/core';
	import Image from '@tiptap/extension-image';
	import Placeholder from '@tiptap/extension-placeholder';
	import Table from '@tiptap/extension-table';
	import TableCell from '@tiptap/extension-table-cell';
	import TableHeader from '@tiptap/extension-table-header';
	import TableRow from '@tiptap/extension-table-row';
	import StarterKit from '@tiptap/starter-kit';
	import { onDestroy, onMount } from 'svelte';
	import { Button } from '../button';
	import { inputVariants } from '../input';
	import { cn } from '$lib';

	type Props = {
		content: object;
		placeholder: string;
		update: (val: object) => void;
		class?: string;
	};

	let { content, placeholder, update, class: className }: Props = $props();
	let element = $state<HTMLElement>();
	let editor = $state<Editor | null>(null);
	let showCommandMenu = $state(false);
	let commandMenuPosition = $state({ x: 0, y: 0 });
	let dropdownElement = $state<HTMLDivElement>();
	let commandInput = $state('');
	let filteredCommands = $state<string[]>([]);
	let inputElement = $state<HTMLInputElement>();
	let selectedIndex = $state(0);

	type FormatMethod =
		| { type: 'chain'; method: (chain: ChainedCommands) => ChainedCommands }
		| { type: 'action'; method: () => void };

	const formatMethods: Record<string, FormatMethod> = {
		Bold: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleBold() },
		Italic: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleItalic() },
		BulletList: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleBulletList() },
		OrderedList: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleOrderedList() },
		H1: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleHeading({ level: 1 }) },
		H2: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleHeading({ level: 2 }) },
		H3: { type: 'chain', method: (chain: ChainedCommands) => chain.toggleHeading({ level: 3 }) },
		Table: {
			type: 'chain',
			method: (chain: ChainedCommands) =>
				chain.insertTable({ rows: 3, cols: 3, withHeaderRow: true })
		},
		Image: { type: 'action', method: handleImageUpload }
	};

	function handleImageUpload() {
		const input = document.createElement('input');
		input.type = 'file';
		input.accept = 'image/*';
		input.onchange = (event) => {
			const file = (event.target as HTMLInputElement).files?.[0];
			if (file) {
				const reader = new FileReader();
				reader.onload = (e) => {
					const result = e.target?.result;
					if (typeof result === 'string' && editor) {
						editor.chain().focus().setImage({ src: result }).run();
					}
				};
				reader.readAsDataURL(file);
			}
		};
		input.click();
	}

	onMount(() => {
		editor = new Editor({
			element,
			extensions: [
				StarterKit,
				Table.configure({ resizable: true }),
				TableRow,
				TableHeader,
				TableCell,
				Image,
				Placeholder.configure({
					placeholder,
					emptyEditorClass: 'is-editor-empty'
				})
			],
			content: content || '',
			editorProps: {
				attributes: {
					class: 'prose prose-sm sm:prose lg:prose-lg xl:prose-2xl focus:outline-none p-4'
				}
			},
			onUpdate: ({ editor }) => {
				const json = editor.getJSON();
				update(json);
			}
		});
		if (!element) return;
		document.addEventListener('keydown', handleKeyDown);
		document.addEventListener('click', handleClickOutside);
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
		document.removeEventListener('keydown', handleKeyDown);
		document.removeEventListener('click', handleClickOutside);
	});

	$effect(() => {
		if (showCommandMenu && inputElement) {
			inputElement.focus();
		}
	});

	function handleKeyDown(event: KeyboardEvent) {
		if (!editor) return;
		if (event.altKey && event.key === 'i') {
			event.preventDefault();
			openCommandMenu();
		} else if (showCommandMenu && event.target === inputElement) {
			switch (event.key) {
				case 'ArrowLeft':
					event.preventDefault();
					selectedIndex = (selectedIndex - 1 + filteredCommands.length) % filteredCommands.length;
					break;
				case 'ArrowRight':
					event.preventDefault();
					selectedIndex = (selectedIndex + 1) % filteredCommands.length;
					break;
				case 'Enter':
					event.preventDefault();
					handleCommand(filteredCommands[selectedIndex]);
					closeCommandMenu();
					break;
				case 'Escape':
					event.preventDefault();
					closeCommandMenu();
					break;
			}
		}
	}

	function openCommandMenu() {
		if (!element || !editor) return;
		const { from } = editor.state.selection;
		const coords = editor.view.coordsAtPos(from);
		const editorRect = element.getBoundingClientRect();
		showCommandMenu = true;
		commandMenuPosition = {
			x: coords.left - editorRect.left,
			y: coords.bottom - editorRect.top
		};
		commandInput = '';
		filteredCommands = Object.keys(formatMethods);
		selectedIndex = 0;
	}

	function closeCommandMenu() {
		if (!editor) return;
		showCommandMenu = false;
		editor.commands.focus();
	}

	function handleClickOutside(event: MouseEvent) {
		if (dropdownElement && !dropdownElement.contains(event.target as Node)) {
			closeCommandMenu();
		}
	}

	function handleCommand(command: string) {
		if (!editor) return;
		const method = formatMethods[command];
		if (method) {
			if (method.type === 'chain') {
				method.method(editor.chain().focus()).run();
			} else {
				method.method();
			}
		}
	}

	function handleInputChange(event: Event) {
		const input = (event.target as HTMLInputElement).value.toLowerCase();
		commandInput = input;
		filteredCommands = Object.keys(formatMethods).filter((cmd) =>
			cmd.toLowerCase().includes(input)
		);
		selectedIndex = 0;
	}
</script>

<div class="rich-text-editor relative w-full">
	<div bind:this={element} class={cn('min-h-[200px] w-full', className)}></div>
	{#if showCommandMenu}
		<div
			bind:this={dropdownElement}
			class="absolute z-[30] rounded-md border border-border bg-card p-2 shadow-lg"
			style="left: {commandMenuPosition.x}px; top: {commandMenuPosition.y}px;"
		>
			<input
				bind:this={inputElement}
				type="text"
				placeholder="Type a command..."
				bind:value={commandInput}
				oninput={handleInputChange}
				class={inputVariants({ variant: 'empty' })}
			/>
			<div class="mt-2 flex flex-wrap">
				{#each filteredCommands as format, index}
					<Button
						variant={index === selectedIndex ? 'secondary' : 'ghost'}
						size="sm"
						class="mb-2 mr-2"
						onclick={() => {
							handleCommand(format);
							closeCommandMenu();
						}}
					>
						{format}
					</Button>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style lang="postcss">
	:global(.ProseMirror) {
		@apply min-h-[200px] outline-none;
	}
	:global(.ProseMirror p.is-editor-empty:first-child::before) {
		@apply pointer-events-none float-left h-0 text-muted-foreground;
		content: attr(data-placeholder);
	}
	:global(.ProseMirror table) {
		@apply w-full table-auto border-collapse;
	}
	:global(.ProseMirror td, .ProseMirror th) {
		@apply relative min-w-[100px] border border-gray-300 p-2;
	}
	:global(.ProseMirror ul) {
		@apply list-disc pl-5;
	}
	:global(.ProseMirror ol) {
		@apply list-decimal pl-5;
	}
	:global(.ProseMirror h1) {
		@apply text-2xl font-bold;
	}
	:global(.ProseMirror h2) {
		@apply text-xl font-semibold;
	}
	:global(.ProseMirror h3) {
		@apply text-lg font-medium;
	}

	:global(.ProseMirror p) {
		@apply text-sm;
	}
</style>
