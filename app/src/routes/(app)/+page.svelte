<script lang="ts">
	import { getIcon } from '$lib';
	import { type StatusIconName, useIssue } from '$lib/api/issue.svelte';
	import { IssueColumnContainer } from '$lib/components/issue';
	import { DefaultWrapper } from '$lib/components/layout';
	import { CreateIssue } from '$lib/components/modals';
	import { Button } from '$lib/components/ui/button';
	import { orgStore } from '$lib/stores/org.store';
	import type { IssueResponse, IssueUpdate } from '$lib/types/issue.type.js';

	let { data } = $props();
	let isCreateIssueModalOpen = $state(false);
	let draggedIssue = $state<IssueResponse | null>(null);
	let dragOverStatus = $state<StatusIconName | null>(null);
	let originalStatus = $state<StatusIconName | null>(null);

	const { createIssue, resp, updateIssue, deleteIssue, createSubIssue, createComment } = useIssue(
		data.accessToken,
		$orgStore.id
	);

	function handleDragStart(e: DragEvent, i: IssueResponse) {
		draggedIssue = i;
		originalStatus = i.issue.status as StatusIconName;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', i.issue.id.toString());
		}
		document.body.classList.add('dragging');
	}

	function handleDragOver(e: DragEvent, status: StatusIconName) {
		e.preventDefault();
		e.stopPropagation();
		if (e.dataTransfer) {
			e.dataTransfer.dropEffect = 'move';
		}
		dragOverStatus = status;
	}

	function handleDragEnd(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		if (draggedIssue && originalStatus !== null) {
			const newStatus = dragOverStatus || originalStatus;
			// Only update if the status has changed
			if (newStatus !== originalStatus) {
				const updatedIssue: IssueUpdate = { status: newStatus };
				updateIssue(draggedIssue.issue.id, updatedIssue);
			}
		}
		draggedIssue = null;
		dragOverStatus = null;
		originalStatus = null;
		document.body.classList.remove('dragging');
	}
</script>

<DefaultWrapper>
	<div class="flex w-full items-center justify-end border-b border-border p-2">
		<Button
			onclick={() => {
				isCreateIssueModalOpen = true;
			}}
			size="sm">Create Issue</Button
		>
	</div>
	<div class="scrollbar h-[calc(100dvh-134px)] overflow-y-auto">
		{#if resp.isLoading}
			<div>Loading...</div>
		{:else if resp.issues.length === 0}
			<div>No issues found</div>
		{:else}
			{#each resp.sortedStatusKeys as status}
				{@const IconStatus = getIcon('status', status as StatusIconName)}
				<IssueColumnContainer
					addComment={createComment}
					{updateIssue}
					org={$orgStore}
					{deleteIssue}
					customId={$orgStore.custom_id}
					issues={resp.issues.filter((i) => i.issue.status === status)}
					{IconStatus}
					{status}
					{handleDragEnd}
					{handleDragStart}
					{handleDragOver}
					issuesCount={resp.statusCount.get(status) || 0}
					{draggedIssue}
					isCreatingIssue={resp.isCreatingIssue}
					onCreateSubIssue={createSubIssue}
				/>
			{/each}
		{/if}
	</div>
</DefaultWrapper>

<CreateIssue
	{createIssue}
	isCreatingIssue={resp.isCreatingIssue}
	bind:isOpen={isCreateIssueModalOpen}
	onClose={() => (isCreateIssueModalOpen = false)}
/>
