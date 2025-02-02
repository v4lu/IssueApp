<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Badge } from '../ui/badge';
	import { Dropdown } from '../ui/dropdown';
	import { Icons } from '../icons';
	import { Editor } from '../ui/editor';
	import { Button } from '../ui/button';
	import { Avatar } from '../ui/avatar';
	import { Input } from '../ui/input';
	import { IssueRowMiniCard } from '.';
	import { type StatusIconName, priorityOrder, statusOrder } from '$lib/api/issue.svelte';
	import type { Org } from '$lib/types/org.type';
	import type { IssueRequest, IssueResponse, IssueUpdate } from '$lib/types/issue.type';
	import { cn, getIcon, getPriorityColor, getStatusColor, getStatusName, monthAndDay } from '$lib';
	import type { CommentRequest } from '$lib/types/comment.type';

	type Props = {
		org: Org;
		issues: IssueResponse[];
		sortedStatusKeys: StatusIconName[];
		updateIssue: (id: string, issue: IssueUpdate) => Promise<void>;
		deleteIssue: (id: string) => Promise<void>;
		addComment: (req: CommentRequest) => Promise<void>;
	};

	let { issues, org, sortedStatusKeys, addComment, updateIssue, deleteIssue }: Props = $props();
	let selectedIssue = $state<IssueResponse | null>(null);
	let isResizing = $state(false);
	let leftPanelWidth = $state(50);
	let resizePreviewWidth = $state(50);
	let animationFrameId = $state<number | null>(null);
	let startX = $state<number>(0);
	let startWidth = $state<number>(0);
	let comment = $state({});
	let statusDropdownOpen = $state(false);
	let priorityDropdownOpen = $state(false);

	function handleIssueSelect(issue: IssueResponse) {
		selectedIssue = issue;
	}

	function startResize(e: MouseEvent) {
		isResizing = true;
		startX = e.clientX;
		startWidth = leftPanelWidth;
		document.body.style.cursor = 'col-resize';
		document.addEventListener('mousemove', handleResize);
		document.addEventListener('mouseup', stopResize);
	}

	function handleResize(e: MouseEvent) {
		if (!isResizing) return;
		if (animationFrameId) cancelAnimationFrame(animationFrameId);

		animationFrameId = requestAnimationFrame(() => {
			const container = document.getElementById('resizable-container');
			if (!container) return;
			const containerWidth = container.offsetWidth;
			const deltaX = e.clientX - startX;
			const deltaPercentage = (deltaX / containerWidth) * 100;
			const newWidth = startWidth + deltaPercentage;
			resizePreviewWidth = Math.min(Math.max(newWidth, 30), 70);
		});
	}

	function stopResize() {
		isResizing = false;
		document.body.style.cursor = '';
		leftPanelWidth = resizePreviewWidth;
		if (animationFrameId) cancelAnimationFrame(animationFrameId);
		document.removeEventListener('mousemove', handleResize);
		document.removeEventListener('mouseup', stopResize);
	}

	function parseJson(description: string | undefined): object {
		if (!description) return {};
		try {
			return JSON.parse(description);
		} catch (error) {
			return {
				type: 'doc',
				content: [{ type: 'paragraph', content: [{ type: 'text', text: description }] }]
			};
		}
	}

	function handleCommentUpdate(content: object) {
		comment = content;
	}

	function handleUpdateField(field: string, value: any) {
		if (!selectedIssue) return;
		updateIssue(selectedIssue.issue.id, { [field]: value });
	}

	function handleAddComment() {
		if (!selectedIssue) return;
		addComment({
			comment_owner_id: selectedIssue.issue.id,
			comment_type: 'Issue',
			content: JSON.stringify(comment)
		});
		comment = {};
	}
</script>

<section
	id="resizable-container"
	class="relative flex h-full w-full gap-0"
	style:user-select={isResizing ? 'none' : 'auto'}
>
	<div
		class="flex transform-gpu flex-col gap-6 overflow-y-auto p-4"
		style:width={`${isResizing ? resizePreviewWidth : leftPanelWidth}%`}
	>
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
		class="relative z-20 flex w-4 cursor-col-resize items-center justify-center"
		role="separator"
		aria-orientation="vertical"
		onpointerdown={startResize}
	>
		<div
			class={cn('group relative h-full w-4 hover:bg-primary/5', isResizing ? 'bg-primary/5' : '')}
		>
			<div
				class={cn(
					'absolute left-1/2 h-full w-px -translate-x-1/2 transition-colors',
					isResizing ? 'bg-primary/50' : 'bg-border/50 group-hover:bg-primary/50'
				)}
			></div>

			<div
				class={cn(
					'absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 transition-opacity',
					isResizing ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'
				)}
			>
				<div class="flex flex-col gap-1">
					<div class="h-1 w-1 rounded-full bg-primary/50"></div>
					<div class="h-1 w-1 rounded-full bg-primary/50"></div>
					<div class="h-1 w-1 rounded-full bg-primary/50"></div>
				</div>
			</div>
		</div>
	</div>

	<div
		class="h-full w-full flex-1 transform-gpu overflow-y-auto p-6"
		style:width={`${isResizing ? 100 - resizePreviewWidth : 100 - leftPanelWidth}%`}
	>
		{#if selectedIssue}
			{@const IconPriority = getIcon('priority', selectedIssue.issue.priority)}
			<div class="grid gap-6">
				<div class="flex items-center justify-between">
					<Badge>{org.custom_id}-{selectedIssue.issue.number}</Badge>
					<div class="flex items-center gap-2">
						<IconPriority class={cn('size-4', getPriorityColor(selectedIssue.issue.priority))} />
						<span class="text-sm">{selectedIssue.issue.priority}</span>
					</div>
				</div>

				<div class="space-y-2">
					<Input
						variant="empty"
						value={selectedIssue.issue.title}
						class="border-0 bg-transparent px-0 text-2xl font-semibold focus-visible:ring-0"
						oninput={(e) =>
							selectedIssue && updateIssue(selectedIssue.issue.id, { title: e.target.value })}
					/>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-2">
						<p class="text-sm font-medium text-muted-foreground">Status</p>
						<Dropdown
							bind:isOpen={statusDropdownOpen}
							triggerText={getStatusName(selectedIssue.issue.status)}
							downArrowIcon
							CustomIcon={Icons.status[selectedIssue.issue.status]}
							customIconClass={cn(getStatusColor(selectedIssue.issue.status), 'size-4')}
							triggerIconPosition="left"
							triggerClass="w-full justify-between px-3 py-2 text-sm border rounded-md hover:bg-muted"
						>
							<div class="p-1">
								{#each statusOrder as status}
									{@const Icon = Icons.status[status]}
									<button
										class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted"
										onclick={() => selectedIssue && updateIssue(selectedIssue.issue.id, { status })}
									>
										<Icon class={cn('size-4', getStatusColor(status))} />
										<span>{getStatusName(status)}</span>
									</button>
								{/each}
							</div>
						</Dropdown>
					</div>

					<div class="space-y-2">
						<p class="text-sm font-medium text-muted-foreground">Priority</p>
						<Dropdown
							bind:isOpen={priorityDropdownOpen}
							triggerText={selectedIssue.issue.priority}
							downArrowIcon
							CustomIcon={Icons.priority[selectedIssue.issue.priority]}
							customIconClass={cn(getPriorityColor(selectedIssue.issue.priority), 'size-4')}
							triggerIconPosition="left"
							triggerClass="w-full justify-between px-3 py-2 text-sm border rounded-md hover:bg-muted"
						>
							<div class="p-1">
								{#each priorityOrder as priority}
									{@const Icon = Icons.priority[priority]}
									<button
										class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted"
										onclick={() =>
											selectedIssue && updateIssue(selectedIssue.issue.id, { priority })}
									>
										<Icon class={cn('size-4', getPriorityColor(priority))} />
										<span>{priority}</span>
									</button>
								{/each}
							</div>
						</Dropdown>
					</div>
				</div>

				<div class="space-y-2">
					<p class="text-sm font-medium text-muted-foreground">Description</p>
					<Editor
						content={parseJson(selectedIssue.issue.description)}
						update={(content) =>
							selectedIssue &&
							updateIssue(selectedIssue.issue.id, { description: JSON.stringify(content) })}
						placeholder="Add a description..."
					/>
				</div>

				<div></div>

				{#if selectedIssue.sub_issues?.length}
					<div class="grid gap-3">
						<h3 class="text-lg font-medium">Sub Issues</h3>
						<div class="grid gap-2">
							{#each selectedIssue.sub_issues as subIssue}
								<IssueRowMiniCard
									issue={subIssue}
									{org}
									IconPriority={getIcon('priority', subIssue.priority)}
									IconStatus={getIcon('status', subIssue.status)}
									customId={org.custom_id}
									{deleteIssue}
									{updateIssue}
								/>
							{/each}
						</div>
					</div>
				{/if}

				<div class="space-y-4">
					<h3 class="text-lg font-medium">Comments</h3>
					<Editor
						placeholder="Add a comment..."
						content={comment}
						class="scrollbar-hidden h-32 max-h-32 min-h-0 overflow-y-auto"
						update={handleCommentUpdate}
					/>
					<div class="flex justify-end">
						<Button
							size="sm"
							onclick={() => {
								if (!selectedIssue) return;
								addComment({
									comment_owner_id: selectedIssue.issue.id,
									comment_type: 'Issue',
									content: JSON.stringify(comment)
								});
								comment = {};
							}}
						>
							Add Comment
						</Button>
					</div>

					{#if selectedIssue.comments?.length}
						<div class="grid gap-4">
							{#each selectedIssue.comments as comment}
								<div class="rounded-lg bg-card p-4 shadow-sm">
									<div class="mb-3 flex items-center gap-4">
										<Avatar user={comment.creator} class="size-12" />
										<div class="flex flex-col">
											<span class="font-medium">{comment.creator.username}</span>
											<span class="text-xs text-muted-foreground">
												{monthAndDay(comment.comment.created_at)}
											</span>
										</div>
									</div>
									<div>{comment.comment.content}</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		{:else}
			<div class="flex h-full w-full items-center justify-center">
				<div class="text-center text-muted-foreground">
					<Icon icon="lucide:inbox" class="mx-auto mb-2 size-12" />
					<p>Select an issue to view details</p>
				</div>
			</div>
		{/if}
	</div>
</section>

<style>
	#resizable-container > div:not(.group) {
		will-change: width;
		transition: width 200ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	#resizable-container:has([aria-orientation='vertical']:active) > div {
		transition: none;
	}

	#resizable-container > div {
		transform: translateZ(0);
		backface-visibility: hidden;
	}
</style>
