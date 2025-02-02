<script lang="ts">
	import { Editor } from '@tiptap/core';
	import StarterKit from '@tiptap/starter-kit';
	import Image from '@tiptap/extension-image';
	import Placeholder from '@tiptap/extension-placeholder';
	import Table from '@tiptap/extension-table';
	import TableRow from '@tiptap/extension-table-row';
	import TableCell from '@tiptap/extension-table-cell';
	import TableHeader from '@tiptap/extension-table-header';
	import Link from '@tiptap/extension-link';
	import TextAlign from '@tiptap/extension-text-align';
	import Highlight from '@tiptap/extension-highlight';
	import TaskList from '@tiptap/extension-task-list';
	import TaskItem from '@tiptap/extension-task-item';
	import CodeBlockLowlight from '@tiptap/extension-code-block-lowlight';
	import Underline from '@tiptap/extension-underline';
	import Icon from '@iconify/svelte';
	import Subscript from '@tiptap/extension-subscript';
	import Superscript from '@tiptap/extension-superscript';
	import TextStyle from '@tiptap/extension-text-style';
	import Color from '@tiptap/extension-color';
	import FontFamily from '@tiptap/extension-font-family';

	import Mention from '@tiptap/extension-mention';

	import { common, createLowlight } from 'lowlight';
	import js from 'highlight.js/lib/languages/javascript';
	import { onMount } from 'svelte';
	import { DefaultWrapper } from '$lib/components/layout';
	import { cn } from '$lib';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';

	const lowlight = createLowlight(common);
	lowlight.register('js', js);

	let editor = $state<Editor | null>(null);
	let showSlashMenu = $state(false);
	let slashMenuPosition = $state({ x: 0, y: 0 });
	let showLinkMenu = $state(false);
	let linkUrl = $state('');
	let title = $state('');

	let tags = $state<string[]>([]);
	let activeView = $state<'tags' | 'ai'>('tags');
	let messages = $state<Array<{ role: 'user' | 'assistant'; content: string }>>([]);
	let userInput = $state('');

	function handleChatInput(event: KeyboardEvent) {
		if (event.key === 'Enter' && userInput.trim()) {
			sendMessage();
		}
	}

	function sendMessage() {
		if (userInput.trim()) {
			messages = [...messages, { role: 'user', content: userInput }];
			// Here you would typically make an API call to your AI service
			// For now, we'll just add a mock response
			setTimeout(() => {
				messages = [
					...messages,
					{
						role: 'assistant',
						content: 'This is a mock AI response. Implement your AI service here.'
					}
				];
			}, 500);
			userInput = '';
		}
	}

	function handleTagInput(event: KeyboardEvent) {
		if (event.key === 'Enter' && event.target && (event.target as HTMLInputElement).value) {
			tags = [...tags, (event.target as HTMLInputElement).value.trim()];
			if (event.target) {
				(event.target as HTMLInputElement).value = '';
			}
		}
	}

	function removeTag(tagToRemove: string) {
		tags = tags.filter((tag) => tag !== tagToRemove);
	}

	let content = $state({
		type: 'doc',
		content: [
			{
				type: 'paragraph',
				content: [{ type: 'text', text: '' }]
			}
		]
	});

	const toolbarGroups = [
		{
			label: 'Formatting',
			items: [
				{
					icon: 'lucide:heading-1',
					title: 'Heading 1',
					action: () => editor?.chain().focus().toggleHeading({ level: 1 }).run(),
					isActive: () => editor?.isActive('heading', { level: 1 }) ?? false
				},
				{
					icon: 'lucide:heading-2',
					title: 'Heading 2',
					action: () => editor?.chain().focus().toggleHeading({ level: 2 }).run(),
					isActive: () => editor?.isActive('heading', { level: 2 }) ?? false
				},
				{
					icon: 'lucide:heading-3',
					title: 'Heading 3',
					action: () => editor?.chain().focus().toggleHeading({ level: 3 }).run(),
					isActive: () => editor?.isActive('heading', { level: 3 }) ?? false
				},
				{
					icon: 'lucide:bold',
					title: 'Bold',
					action: () => editor?.chain().focus().toggleBold().run(),
					isActive: () => editor?.isActive('bold') ?? false
				},
				{
					icon: 'lucide:italic',
					title: 'Italic',
					action: () => editor?.chain().focus().toggleItalic().run(),
					isActive: () => editor?.isActive('italic') ?? false
				},
				{
					icon: 'lucide:underline',
					title: 'Underline',
					action: () => editor?.chain().focus().toggleStrike().run(), // Should be toggleUnderline
					isActive: () => editor?.isActive('strike') ?? false // Should be 'underline'
				},
				{
					icon: 'lucide:highlighter',
					title: 'Highlight',
					action: () => editor?.chain().focus().toggleHighlight().run(),
					isActive: () => editor?.isActive('highlight') ?? false
				}
			]
		},
		{
			label: 'Alignment',
			items: [
				{
					icon: 'lucide:align-left',
					title: 'Align Left',
					action: () => editor?.chain().focus().setTextAlign('left').run(),
					isActive: () => editor?.isActive({ textAlign: 'left' }) ?? false
				},
				{
					icon: 'lucide:align-center',
					title: 'Align Center',
					action: () => editor?.chain().focus().setTextAlign('center').run(),
					isActive: () => editor?.isActive({ textAlign: 'center' }) ?? false
				},
				{
					icon: 'lucide:align-right',
					title: 'Align Right',
					action: () => editor?.chain().focus().setTextAlign('right').run(),
					isActive: () => editor?.isActive({ textAlign: 'right' }) ?? false
				}
			]
		},
		{
			label: 'Lists',
			items: [
				{
					icon: 'lucide:list',
					title: 'Bullet List',
					action: () => editor?.chain().focus().toggleBulletList().run(),
					isActive: () => editor?.isActive('bulletList') ?? false
				},
				{
					icon: 'lucide:list-ordered',
					title: 'Numbered List',
					action: () => editor?.chain().focus().toggleOrderedList().run(),
					isActive: () => editor?.isActive('orderedList') ?? false
				},
				{
					icon: 'lucide:check-square',
					title: 'Task List',
					action: () => editor?.chain().focus().toggleTaskList().run(),
					isActive: () => editor?.isActive('taskList') ?? false
				}
			]
		},
		{
			label: 'Insert',
			items: [
				{
					icon: 'lucide:table',
					title: 'Insert Table',
					action: () => editor?.chain().focus().insertTable({ rows: 3, cols: 3 }).run()
				},
				{
					icon: 'lucide:image',
					title: 'Insert Image',
					action: () => {
						const url = window.prompt('Enter image URL');
						if (url) {
							editor?.chain().focus().setImage({ src: url }).run();
						}
					}
				},
				{
					icon: 'lucide:link',
					title: 'Insert Link',
					action: () => {
						showLinkMenu = true;
					},
					isActive: () => editor?.isActive('link') ?? false
				},
				{
					icon: 'lucide:code',
					title: 'Code Block',
					action: () => editor?.chain().focus().toggleCodeBlock().run(),
					isActive: () => editor?.isActive('codeBlock') ?? false
				}
			]
		},
		{
			label: 'Advanced Formatting',
			items: [
				{
					icon: 'lucide:palette',
					title: 'Text Color',
					action: () => {
						const color = window.prompt('Enter color');
						if (color) {
							editor?.chain().focus().setColor(color).run();
						}
					}
				}
			]
		}
	];

	const formatOptions = [
		{ label: 'Paragraph', icon: 'lucide:text' },
		{
			label: 'Heading 1',
			icon: 'lucide:heading-1',
			command: () => editor?.chain().focus().toggleHeading({ level: 1 }).run()
		},
		{
			label: 'Heading 2',
			icon: 'lucide:heading-2',
			command: () => editor?.chain().focus().toggleHeading({ level: 2 }).run()
		},
		{
			label: 'Heading 3',
			icon: 'lucide:heading-3',
			command: () => editor?.chain().focus().toggleHeading({ level: 3 }).run()
		},
		{
			label: 'Quote',
			icon: 'lucide:quote',
			command: () => editor?.chain().focus().toggleBlockquote().run()
		},
		{
			label: 'Code Block',
			icon: 'lucide:code',
			command: () => editor?.chain().focus().toggleCodeBlock().run()
		},
		{
			label: 'Task List',
			icon: 'lucide:check-square',
			command: () => editor?.chain().focus().toggleTaskList().run()
		},
		{
			label: 'Table',
			icon: 'lucide:table',
			command: () => editor?.chain().focus().insertTable({ rows: 3, cols: 3 }).run()
		}
	];

	onMount(() => {
		const editorElement = document.querySelector('#editor');
		if (editorElement) {
			editor = new Editor({
				element: editorElement,
				extensions: [
					StarterKit.configure({
						heading: {
							levels: [1, 2, 3]
						}
					}),
					Image,
					Underline,
					Placeholder.configure({
						placeholder: 'Type / for commands...'
					}),
					Table.configure({
						resizable: true
					}),
					TableRow,
					TableCell,
					TableHeader,
					Link.configure({
						openOnClick: true,
						linkOnPaste: true
					}),
					TextAlign.configure({
						types: ['heading', 'paragraph']
					}),
					Highlight,
					TaskList,
					TaskItem.configure({
						nested: true
					}),
					CodeBlockLowlight.configure({
						lowlight
					}),
					Subscript,
					Superscript,
					Color,
					FontFamily,
					Mention,
					Color.configure({
						types: ['textStyle'],
						defaultColor: 'inherit'
					}),
					TextStyle
				],
				content,
				editorProps: {
					attributes: {
						class: 'focus:outline-none max-w-full'
					},

					handleKeyDown: (view, event) => {
						if (event.key === '/') {
							const coords = view.coordsAtPos(view.state.selection.from);
							const editorRect = view.dom.getBoundingClientRect();

							slashMenuPosition = {
								x: coords.left - editorRect.left,
								y: coords.top - editorRect.top + 48
							};
							showSlashMenu = true;

							return false;
						}

						if (showSlashMenu && event.key === 'Escape') {
							showSlashMenu = false;
							return true;
						}

						return false;
					}
				}
			});
		}

		return () => {
			editor?.destroy();
		};
	});

	function setLink() {
		if (linkUrl) {
			editor?.chain().focus().setLink({ href: linkUrl }).run();
		}
		showLinkMenu = false;
		linkUrl = '';
	}
</script>

<DefaultWrapper class="grid h-[92.5dvh] max-h-[92.5dvh]">
	<div class="rounded-lg shadow-sm">
		<div class="border-b border-border p-4">
			<Input
				type="text"
				placeholder="Enter title..."
				class={cn('text-2xl')}
				variant="empty"
				bind:value={title}
			/>
		</div>
		<div class="border-b border-border bg-muted/50 p-2">
			<div class="flex items-center gap-4">
				{#each toolbarGroups as group}
					<div class="flex items-center gap-1">
						{#each group.items as item}
							<button
								class={cn(
									'rounded-md p-2 transition-colors hover:bg-muted',
									item.isActive?.() && 'bg-muted'
								)}
								title={item.title}
								onclick={item.action}
							>
								<Icon icon={item.icon} class="size-4" />
							</button>
						{/each}
					</div>
					<div class="h-6 w-px bg-border"></div>
				{/each}
			</div>
		</div>

		<div
			class="scrollbar-hidden relative h-[82dvh] max-h-[84dvh] w-full flex-1 overflow-y-auto p-4"
		>
			<div id="editor" class="h-full"></div>

			{#if showSlashMenu}
				<div
					class="absolute z-50 w-64 rounded-lg border border-border bg-card shadow-lg"
					style="left: {slashMenuPosition.x}px; top: {slashMenuPosition.y}px;"
				>
					<div class="p-1">
						{#each formatOptions as option}
							<button
								class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm hover:bg-muted"
								onclick={() => {
									showSlashMenu = false;
									option.command?.();
								}}
							>
								<Icon icon={option.icon} class="size-4" />
								<span>{option.label}</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</div>
</DefaultWrapper>
<div class="px-4 pb-4">
	<!-- View Toggle -->
	<div class="mb-4 flex space-x-2 border-b border-border">
		<button
			class={cn(
				'px-4 py-2 text-sm transition-colors',
				activeView === 'tags'
					? 'border-b-2 border-primary font-medium'
					: 'text-muted-foreground hover:text-foreground'
			)}
			onclick={() => (activeView = 'tags')}
		>
			Tags
		</button>
		<button
			class={cn(
				'px-4 py-2 text-sm transition-colors',
				activeView === 'ai'
					? 'border-b-2 border-primary font-medium'
					: 'text-muted-foreground hover:text-foreground'
			)}
			onclick={() => (activeView = 'ai')}
		>
			AI Chat
		</button>
	</div>

	{#if activeView === 'tags'}
		<!-- Tags View -->
		<div class="space-y-4">
			<div class="mb-4 space-y-2">
				<h3 class="font-medium text-foreground">Tags</h3>
				<p class="text-sm text-muted-foreground">Add tags to organize your content</p>
			</div>

			<div class="mb-3">
				<Input
					type="text"
					placeholder="Type tag and press Enter..."
					class="h-9 w-full text-sm placeholder:text-sm"
					onkeydown={handleTagInput}
				/>
			</div>

			<div class="flex flex-wrap gap-2">
				{#each tags as tag}
					<Badge class="px-2 py-1.5 text-sm transition-opacity hover:opacity-100">
						{tag}
					</Badge>
				{/each}
			</div>
		</div>
	{:else}
		<!-- AI Chat View -->
		<div class="space-y-4">
			<div class="mb-4 space-y-2">
				<h3 class="font-medium text-foreground">AI Assistant</h3>
				<p class="text-sm text-muted-foreground">Ask questions about your content</p>
			</div>

			<div class="space-y-4">
				<!-- Chat Messages -->
				<div
					class="scrollbar-hidden h-[78dvh] max-h-[78dvh] space-y-4 overflow-y-auto rounded-lg border border-border p-4"
				>
					{#each messages as message}
						<div
							class={cn(
								'flex gap-2 rounded-lg p-3',
								message.role === 'user' ? 'bg-muted' : 'bg-muted/50'
							)}
						>
							<span class="text-sm">{message.content}</span>
						</div>
					{/each}
				</div>

				<!-- Input Area -->
				<div class="flex gap-2">
					<Input
						type="text"
						placeholder="Ask a question..."
						class="h-9 text-sm"
						bind:value={userInput}
						onkeydown={handleChatInput}
					/>
					<Button class="h-9" onclick={sendMessage}>Send</Button>
				</div>
			</div>
		</div>
	{/if}
</div>

{#if showLinkMenu}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
		<div class="w-96 rounded-lg bg-card p-4 shadow-lg">
			<h3 class="mb-4 text-lg font-medium">Insert Link</h3>
			<input
				type="url"
				bind:value={linkUrl}
				placeholder="Enter URL"
				class="mb-4 w-full rounded-md border p-2"
			/>
			<div class="flex justify-end gap-2">
				<button
					class="rounded-md bg-muted px-3 py-1"
					onclick={() => {
						showLinkMenu = false;
						linkUrl = '';
					}}
				>
					Cancel
				</button>
				<button class="rounded-md bg-primary px-3 py-1 text-primary-foreground" onclick={setLink}>
					Insert
				</button>
			</div>
		</div>
	</div>
{/if}

<style lang="postcss">
	:global(.ProseMirror) {
		@apply h-full w-full max-w-full p-4 focus:outline-none;
	}

	:global(.ProseMirror table) {
		@apply m-0 w-full border-collapse;
	}

	:global(.ProseMirror td),
	:global(.ProseMirror th) {
		@apply relative box-border min-w-[1em] border-2 border-border p-2 align-top;
	}

	:global(.ProseMirror th) {
		@apply bg-background-muted text-left font-bold;
	}

	:global(.ProseMirror img) {
		@apply h-auto max-w-full;
	}

	:global(.ProseMirror ul) {
		@apply my-4 list-disc pl-5;
	}

	:global(.ProseMirror ol) {
		@apply my-4 list-decimal pl-5;
	}

	:global(.ProseMirror h1) {
		@apply mb-4 text-3xl font-bold text-foreground;
	}

	:global(.ProseMirror h2) {
		@apply mb-3 text-2xl font-semibold text-foreground;
	}

	:global(.ProseMirror h3) {
		@apply mb-2 text-xl font-medium text-foreground;
	}

	:global(.ProseMirror p) {
		@apply text-base leading-relaxed text-foreground;
	}

	:global(.ProseMirror blockquote) {
		@apply my-4 border-l-4 border-border pl-4 italic text-muted-foreground;
	}

	:global(.ProseMirror code) {
		@apply rounded-md bg-muted px-1.5 py-0.5 font-mono text-sm;
	}

	:global(.ProseMirror pre) {
		@apply my-4 overflow-x-auto rounded-lg bg-muted p-4;
	}

	:global(.ProseMirror a) {
		@apply text-primary hover:underline;
	}

	:global(.ProseMirror hr) {
		@apply my-6 border-border;
	}

	:global(.ProseMirror mark) {
		@apply bg-accent text-accent-foreground;
	}
</style>
