<script lang="ts">
	import type { Component } from 'svelte';
	import { DeleteModal } from '../modals';
	import {
		cn,
		formatDateOrDefault,
		getPriorityColor,
		getStatusColor,
		isPastToday,
		monthAndDay
	} from '$lib';
	import type { Issue, IssueUpdate } from '$lib/types/issue.type';
	import type { Org } from '$lib/types/org.type';
	import { contextMenuStore } from '$lib/stores/issue-row-card-context-menu.store';

	type Props = {
		issue: Issue;
		IconPriority: Component;
		IconStatus: Component;
		customId: string;
		updateIssue: (id: string, issue: IssueUpdate) => Promise<void>;
		deleteIssue: (id: string) => Promise<void>;
		org: Org;
	};

	let { issue, IconPriority, IconStatus, customId, deleteIssue, org }: Props = $props();

	let showDeleteConfirmation = $state(false);

	function handleContextMenu(event: MouseEvent) {
		event.preventDefault();
		const target = event.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();

		const viewportWidth = window.innerWidth;
		const viewportHeight = window.innerHeight;

		const menuWidth = 210;
		const menuHeight = 200;

		contextMenuStore.update((state) => {
			if (state.activeId === issue.id) {
				return {
					activeId: null,
					x: 0,
					y: 0
				};
			}

			let posX = event.clientX;
			let posY = rect.top + rect.height;

			if (posX + menuWidth > viewportWidth) {
				posX = viewportWidth - menuWidth - 40;
			}

			if (posY + menuHeight > viewportHeight) {
				posY = rect.top - menuHeight;
			}

			posX = Math.max(10, posX);
			posY = Math.max(10, posY);

			return {
				activeId: issue.id,
				x: posX,
				y: posY
			};
		});
	}
</script>

<div
	tabindex="0"
	role="button"
	onkeydown={() => {}}
	aria-roledescription="extra actions for issue"
	oncontextmenu={handleContextMenu}
	class="flex gap-4 border-b border-border px-4 py-2 transition-colors hover:bg-accent/50"
>
	<IconPriority class={cn('size-5', getPriorityColor(issue.priority))} />
	<p class="truncate text-xs text-muted-foreground">{`${customId}-${issue.number}`}</p>
	<IconStatus class={cn('size-5', getStatusColor(issue.status))} />
	<p class=" max-w-[22rem] truncate text-sm font-medium">{issue.title}</p>
	<div class="flex items-center gap-2">
		{#if issue.due_date && formatDateOrDefault(issue.due_date) !== '-'}
			<p
				class={cn(
					' text-xs font-semibold text-muted-foreground',
					formatDateOrDefault(issue.due_date) !== '-' &&
						isPastToday(issue.due_date) &&
						'text-destructive'
				)}
			>
				{monthAndDay(issue.due_date)}
			</p>
		{/if}
	</div>
</div>

<DeleteModal
	onConfirm={() => {
		deleteIssue(issue.id);
		showDeleteConfirmation = false;
	}}
	bind:isOpen={showDeleteConfirmation}
	title={`Are you sure you want to delete the issue ${org.custom_id}-${issue.number}`}
	description="This action cannot be undone."
/>
