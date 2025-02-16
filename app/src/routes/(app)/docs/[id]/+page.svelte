<script lang="ts">
	import { Editor } from '@tiptap/core';
	import { DefaultWrapper } from '$lib/components/layout';
	import { cn } from '$lib';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { DocEditor } from '$lib/components/editor';

	let editor = $state<Editor | null>(null);
	let title = $state('');
	let activeView = $state<'ai' | 'info'>('info');
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

	function handleDocSave() {
		const val = editor?.$doc?.content.toJSON();
		console.log({ title, val });
	}
</script>

<DefaultWrapper class="grid h-[92.5dvh] max-h-[92.5dvh]">
	<DocEditor bind:title bind:editor />
</DefaultWrapper>
<div class="px-4 pb-4">
	<div class="mb-4 flex space-x-2 border-b border-border">
		<button
			class={cn(
				'px-4 py-2 text-sm transition-colors',
				activeView === 'info'
					? 'border-b-2 border-primary font-medium'
					: 'text-muted-foreground hover:text-foreground'
			)}
			onclick={() => (activeView = 'info')}
		>
			Info
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
	{#if activeView === 'info'}
		<div class="space-y-4">
			<div class="mb-4 space-y-2">
				<h3 class="font-medium text-foreground">Info</h3>

				<Button onclick={handleDocSave}>Save</Button>
			</div>
		</div>
	{:else if activeView === 'ai'}
		<div class="space-y-4">
			<div class="mb-4 space-y-2">
				<h3 class="font-medium text-foreground">AI Assistant</h3>
				<p class="text-sm text-muted-foreground">Ask questions about your content</p>
			</div>

			<div class="space-y-4">
				<div
					class="scrollbar-hidden h-[78dvh] max-h-[78dvh] space-y-4 overflow-y-auto rounded-lg border border-border p-4"
				>
					{#each messages as message}
						<div
							class={cn(
								'flex gap-2 rounded-lg p-3',
								message.role === 'user' ? 'bg-accent' : 'bg-accent/50'
							)}
						>
							<span class="text-sm">{message.content}</span>
						</div>
					{/each}
				</div>

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
