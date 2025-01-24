<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { StatusIconName } from '$lib/api/issue.svelte';
	import type { IssueResponse } from '$lib/types/issue.type';
	import type { Org } from '$lib/types/org.type';
	import { cn, getIcon, getPriorityColor } from '$lib';

	type Props = {
		org: Org;
		issues: IssueResponse[];
		sortedStatusKeys: StatusIconName[];
	};

	let { issues, org, sortedStatusKeys }: Props = $props();
	let selectedIssue = $state<IssueResponse | null>(null);
	let isResizing = $state(false);
	let leftPanelWidth = $state(50);

	function handleIssueSelect(issue: IssueResponse) {
		selectedIssue = issue;
	}

	function startResize(e: MouseEvent) {
		isResizing = true;
		document.addEventListener('mousemove', handleResize);
		document.addEventListener('mouseup', stopResize);
	}

	function handleResize(e: MouseEvent) {
		if (!isResizing) return;

		const container = document.getElementById('resizable-container');
		if (!container) return;

		const containerRect = container.getBoundingClientRect();
		const newWidth = ((e.clientX - containerRect.left) / containerRect.width) * 100;

		leftPanelWidth = Math.min(Math.max(newWidth, 30), 70);
	}

	function stopResize() {
		isResizing = false;
		document.removeEventListener('mousemove', handleResize);
		document.removeEventListener('mouseup', stopResize);
	}
</script>

<section
	id="resizable-container"
	class="relative flex h-full gap-0"
	style:user-select={isResizing ? 'none' : 'auto'}
>
	<div class="overflow-y-auto p-4" style:width={`${leftPanelWidth}%`}>
		{#each sortedStatusKeys as status}
			{@const IconStatus = getIcon('status', status as StatusIconName)}
			<div class="grid gap-3">
				<div class="flex items-center justify-start gap-3 px-2">
					<IconStatus
						class={cn(
							'size-5',
							status === 'InProgress' && 'text-yellow-500 dark:text-yellow-400',
							status === 'InReview' && 'text-blue-500 dark:text-blue-400',
							status === 'Blocked' && 'text-destructive',
							status === 'Canceled' && 'text-destructive',
							status === 'Done' && 'text-emerald-600 dark:text-emerald-400',
							status === 'Todo' && 'text-blue-600 dark:text-blue-400',
							status === 'Backlog' && 'text-purple-600 dark:text-purple-400'
						)}
					/>
					<h3 class="text-sm font-semibold">
						{status} ({issues.filter((i) => i.issue.status === status).length})
					</h3>
				</div>

				<div class="grid gap-2">
					{#each issues.filter((i) => i.issue.status === status) as issue}
						{@const IconPriority = getIcon('priority', issue.issue.priority)}
						<button
							class={cn(
								'flex items-center justify-between rounded-lg border border-border bg-card p-4 transition-all hover:border-primary/50 hover:shadow-sm',
								selectedIssue?.issue_id === issue.issue_id && 'border-primary/50 bg-primary/5'
							)}
							onclick={() => handleIssueSelect(issue)}
						>
							<div class="grid gap-y-3">
								<div class="flex items-center justify-start gap-2">
									<IconPriority class={cn('size-4', getPriorityColor(issue.issue.priority))} />
									<span class="text-xs text-muted-foreground">
										NAM-{issue.issue.number}
									</span>
								</div>
								<h3 class="text-left text-base font-medium">{issue.issue.title}</h3>
							</div>
							<Icon icon="lucide:chevron-right" class="size-5 text-muted-foreground" />
						</button>
					{/each}
				</div>
			</div>
		{/each}
	</div>

	<div
		class="relative z-10 flex cursor-col-resize items-center justify-center"
		onmousedown={startResize}
	>
		<div
			class="overflow-y-auto border-l border-border p-6"
			style:width={`${100 - leftPanelWidth}%`}
		>
			<div class="absolute top-1/2 h-8 w-1 -translate-y-1/2 bg-border hover:bg-primary/50"></div>
		</div>
	</div>

	<div class=" w-full border-l border-border p-6">
		{#if selectedIssue}
			{@const IconPriority = getIcon('priority', selectedIssue.issue.priority)}
			<div class="grid gap-6">
				<div class="flex items-center justify-between">
					<span class="text-sm text-muted-foreground">NAM-{selectedIssue.issue.number}</span>
					<div class="flex items-center gap-2">
						<IconPriority class={cn('size-4', getPriorityColor(selectedIssue.issue.priority))} />
						<span class="text-sm">{selectedIssue.issue.priority}</span>
					</div>
				</div>

				<h2 class="text-2xl font-semibold">{selectedIssue.issue.title}</h2>

				<div class="prose prose-sm dark:prose-invert">
					<p>{selectedIssue.issue.description}</p>
				</div>

				{#if selectedIssue.sub_issues?.length}
					<div class="grid gap-3">
						<h3 class="text-lg font-medium">Sub Issues</h3>
						<div class="grid gap-2">
							{#each selectedIssue.sub_issues as subIssue}
								<div class="rounded-md border border-border p-3">
									<span class="text-sm">{subIssue.title}</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				{#if selectedIssue.comments?.length}
					<div class="grid gap-3">
						<h3 class="text-lg font-medium">Comments</h3>
						<div class="grid gap-2">
							{#each selectedIssue.comments as comment}
								<div class="rounded-md border border-border p-3">
									<p class="text-sm">{comment.comment.content}</p>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{:else}
			<div class="grid h-full place-items-center">
				<div class="text-center text-muted-foreground">
					<Icon icon="lucide:inbox" class="mx-auto mb-2 size-12" />
					<p>Select an issue to view details</p>
				</div>
			</div>
		{/if}
	</div>
</section>
