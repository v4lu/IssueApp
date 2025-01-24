<script lang="ts">
	import { type StatusIconName, useIssue } from '$lib/api/issue.svelte';
	import { IssueColumnContainer, IssueNewViewContainer } from '$lib/components/issue';
	import { DefaultWrapper } from '$lib/components/layout';
	import { Button } from '$lib/components/ui/button';
	import { orgStore } from '$lib/stores/org.store';
	import type { IssueResponse, IssueUpdate } from '$lib/types/issue.type.js';
	import { CreateIssue } from '$lib/components/modals';
	import { getIcon } from '$lib';

	type SelectedView = 'table' | 'mail' | 'kanban';

	let { data } = $props();
	let isCreateIssueModalOpen = $state(false);
	let selectedView = $state<SelectedView>('mail');

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
	<div class="flex w-full items-center justify-end gap-4 border-b border-border p-2">
		<button onclick={() => (selectedView = 'table')}>table view</button>
		<button onclick={() => (selectedView = 'mail')}>mail view</button>
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
		{:else if selectedView === 'table'}
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
		{:else if selectedView === 'mail'}
			<IssueNewViewContainer
				org={$orgStore}
				sortedStatusKeys={resp.sortedStatusKeys}
				issues={resp.issues}
			/>
		{/if}
	</div>
</DefaultWrapper>

<CreateIssue
	{createIssue}
	isCreatingIssue={resp.isCreatingIssue}
	bind:isOpen={isCreateIssueModalOpen}
	onClose={() => (isCreateIssueModalOpen = false)}
/>
