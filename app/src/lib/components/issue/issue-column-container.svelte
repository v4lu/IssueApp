<script lang="ts">
	import type { Component } from 'svelte';
	import { IssueRowCard } from '.';
	import { cn, getIcon, getStatusName } from '$lib';
	import type { StatusIconName } from '$lib/api/issue.svelte';
	import type { IssueRequest, IssueResponse, IssueUpdate } from '$lib/types/issue.type';
	import type { Org } from '$lib/types/org.type';
	import type { CommentRequest } from '$lib/types/comment.type';

	type Props = {
		IconStatus: Component;
		issuesCount: number;
		status: StatusIconName;
		issues: IssueResponse[];
		customId: string;
		updateIssue: (issueId: string, issue: IssueUpdate) => Promise<void>;
		deleteIssue: (id: string) => Promise<void>;
		handleDragStart: (e: DragEvent, issue: IssueResponse) => void;
		handleDragEnd: (e: DragEvent) => void;
		handleDragOver: (e: DragEvent, status: StatusIconName) => void;
		draggedIssue: IssueResponse | null;
		org: Org;
		isCreatingIssue: boolean;
		onCreateSubIssue: (issue: IssueRequest) => Promise<void>;
		addComment: (req: CommentRequest) => Promise<void>;
	};

	let {
		addComment,
		IconStatus,
		issuesCount,
		status,
		issues,
		customId,
		draggedIssue,
		handleDragEnd,
		handleDragOver,
		handleDragStart,
		updateIssue,
		deleteIssue,
		org,
		onCreateSubIssue,
		isCreatingIssue
	}: Props = $props();
</script>

<section class="w-full">
	<div
		class="sticky top-0 z-10 flex h-fit w-full items-center justify-start gap-4 bg-card px-4 py-2 lg:px-8"
	>
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
		<h2 class="text-xs font-medium">
			{getStatusName(status)} ({issuesCount})
		</h2>
	</div>
	<div
		ondragover={(e) => handleDragOver(e, status)}
		ondrop={(e) => handleDragEnd(e)}
		role="article"
	>
		{#each issues as issue}
			<IssueRowCard
				{addComment}
				{customId}
				{issue}
				{org}
				IconPriority={getIcon('priority', issue.issue.priority)}
				IconStatus={getIcon('status', issue.issue.status)}
				{draggedIssue}
				{deleteIssue}
				{updateIssue}
				{handleDragEnd}
				{handleDragStart}
				{isCreatingIssue}
				{onCreateSubIssue}
			/>
		{/each}
	</div>
</section>
