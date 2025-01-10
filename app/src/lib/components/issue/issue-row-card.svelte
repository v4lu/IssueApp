<script lang="ts">
	import type { Component } from 'svelte';
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { ContextMenu } from '../ui/context-menu';
	import { Button } from '../ui/button';
	import { DeleteModal } from '../modals';
	import { Icons } from '../icons';
	import { IssuePannel } from '.';
	import {
		cn,
		formatDateOrDefault,
		getPriorityColor,
		getStatusColor,
		isPastToday,
		monthAndDay
	} from '$lib';
	import type { IssueRequest, IssueResponse, IssueUpdate } from '$lib/types/issue.type';
	import { priorityOrder, statusOrder } from '$lib/api/issue.svelte';
	import type { Org } from '$lib/types/org.type';
	import { contextMenuStore } from '$lib/stores/issue-row-card-context-menu.store';
	import type { CommentRequest } from '$lib/types/comment.type';

	type MenuType = 'status' | 'priority';
	type Props = {
		issue: IssueResponse;
		IconPriority: Component;
		IconStatus: Component;
		customId: string;
		draggedIssue: IssueResponse | null;
		updateIssue: (id: string, issue: IssueUpdate) => Promise<void>;
		deleteIssue: (id: string) => Promise<void>;
		handleDragStart: (e: DragEvent, issue: IssueResponse) => void;
		handleDragEnd: (e: DragEvent) => void;
		org: Org;
		isCreatingIssue: boolean;
		onCreateSubIssue: (issue: IssueRequest) => Promise<void>;
		addComment: (req: CommentRequest) => Promise<void>;
	};

	let {
		issue,
		IconPriority,
		IconStatus,
		customId,
		deleteIssue,
		draggedIssue,
		handleDragEnd,
		handleDragStart,
		updateIssue,
		org,
		onCreateSubIssue,
		isCreatingIssue,
		addComment
	}: Props = $props();

	const menuItems: Array<{ type: MenuType; icon: string; text: string; items: string[] }> = $state([
		{
			type: 'status',
			icon: 'solar:sort-from-top-to-bottom-bold',
			text: 'Change Status',
			items: statusOrder
		},
		{ type: 'priority', icon: 'solar:flag-bold', text: 'Change Priority', items: priorityOrder }
	]);
	let showDeleteConfirmation = $state(false);

	let activeSubmenu = $state<MenuType | null>(null);
	let submenuTimeout = $state<number | null>(null);
	let isOpenPanel = $state(false);
	let activeContextMenu = $state<string | null>(null);
	let menuX = $state(0);
	let menuY = $state(0);

	function createDragImage(i: IssueResponse) {
		const dragEl = document.createElement('div');
		dragEl.className =
			'fixed left-0 top-0 z-50 w-[300px] rounded-md border border-border bg-card p-4 shadow-lg';

		dragEl.innerHTML = `
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted-foreground">${customId}-${i.issue.number}</span>
        <span class="text-sm font-medium truncate">${i.issue.title}</span>
      </div>
    `;

		document.body.appendChild(dragEl);
		return dragEl;
	}

	function handleDragStartCustom(e: DragEvent, i: IssueResponse) {
		const dragImage = createDragImage(issue);

		if (e.dataTransfer) {
			e.dataTransfer.setDragImage(dragImage, 150, 25);
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', i.issue.id.toString());
		}

		setTimeout(() => dragImage.remove(), 0);

		handleDragStart(e, issue);
	}

	function handleContextMenu(event: MouseEvent) {
		event.preventDefault();
		const target = event.currentTarget as HTMLElement;
		const rect = target.getBoundingClientRect();

		const viewportWidth = window.innerWidth;
		const viewportHeight = window.innerHeight;

		const menuWidth = 210;
		const menuHeight = 200;

		contextMenuStore.update((state) => {
			if (state.activeId === issue.issue.id) {
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
				activeId: issue.issue.id,
				x: posX,
				y: posY
			};
		});
	}

	function handleSubmenu(submenu: MenuType, isEnter: boolean) {
		if (submenuTimeout) clearTimeout(submenuTimeout);

		if (isEnter) {
			activeSubmenu = submenu;
		} else {
			submenuTimeout = setTimeout(() => {
				activeSubmenu = null;
			}, 300) as unknown as number;
		}
	}

	function closeContextMenu() {
		contextMenuStore.set({
			activeId: null,
			x: 0,
			y: 0
		});
		activeSubmenu = null;
	}

	function getIcon(type: MenuType, item: string): Component {
		if (type === 'status') {
			return Icons.status[item as keyof typeof Icons.status];
		} else {
			return Icons.priority[item as keyof typeof Icons.priority];
		}
	}

	contextMenuStore.subscribe((state) => {
		activeContextMenu = state.activeId;
		menuX = state.x;
		menuY = state.y;
	});
</script>

<svelte:window onclick={closeContextMenu} onresize={closeContextMenu} />

<div
	tabindex="0"
	role="button"
	onkeydown={() => {}}
	aria-roledescription="extra actions for issue"
	oncontextmenu={handleContextMenu}
	draggable="true"
	onclick={() => (isOpenPanel = true)}
	ondragstart={(e) => handleDragStartCustom(e, issue)}
	ondragend={handleDragEnd}
	class="grid cursor-move grid-cols-[auto_auto_auto_1fr_auto] items-center gap-4 border-b border-border px-4 py-2 transition-colors hover:bg-accent/50 lg:px-8"
	class:opacity-50={draggedIssue?.issue?.id === issue.issue.id}
>
	<IconPriority class={cn('size-5', getPriorityColor(issue.issue.priority))} />
	<p class="w-20 truncate text-xs text-muted-foreground">{`${customId}-${issue.issue.number}`}</p>
	<IconStatus class={cn('size-5', getStatusColor(issue.issue.status))} />
	<p class="w-fit text-sm font-medium">{issue.issue.title}</p>
	<div class="flex items-center gap-2">
		{#if issue.issue.due_date && formatDateOrDefault(issue.issue.due_date) !== '-'}
			<p
				class={cn(
					'text-xs font-semibold text-muted-foreground',
					formatDateOrDefault(issue.issue.due_date) !== '-' &&
						isPastToday(issue.issue.due_date) &&
						'text-destructive'
				)}
			>
				{monthAndDay(issue.issue.due_date)}
			</p>
		{/if}
	</div>
</div>

<ContextMenu
	isOpen={activeContextMenu === issue.issue.id}
	x={menuX}
	y={menuY}
	class="min-w-[12rem] p-0"
	onclick={(e) => e.stopPropagation()}
>
	{#each menuItems as menu}
		<div
			tabindex="0"
			role="button"
			aria-roledescription="extra actions for issue {menu.type}"
			class="relative"
			onmouseenter={() => handleSubmenu(menu.type, true)}
			onmouseleave={() => handleSubmenu(menu.type, false)}
		>
			<Button variant="ghost" size="sm" class="w-full justify-between rounded-none">
				<span class="flex items-center">
					<Icon icon={menu.icon} class="mr-2 size-5" />
					{menu.text}
				</span>
				<Icon icon="lucide:chevron-right" class="size-4" />
			</Button>
			{#if activeSubmenu === menu.type}
				<div
					tabindex="0"
					role="button"
					aria-roledescription="select new {menu.type}"
					class="absolute left-full top-0 ml-1 w-40 rounded-md border border-border bg-card shadow-md"
					transition:fade={{ duration: 300 }}
					onmouseenter={() => handleSubmenu(menu.type, true)}
					onmouseleave={() => handleSubmenu(menu.type, false)}
				>
					{#each menu.items as item}
						{@const IconComponent = getIcon(menu.type, item)}
						<Button
							onclick={() => {
								updateIssue(issue.issue.id, { [menu.type]: item });
								closeContextMenu();
							}}
							variant="ghost"
							class="flex w-full justify-start rounded-none first:rounded-t-md last:rounded-b-md"
							size="sm"
						>
							<IconComponent
								class={cn(
									'mr-2 size-4',
									menu.type === 'status' ? getStatusColor(item) : getPriorityColor(item)
								)}
							/>
							{item}
						</Button>
					{/each}
				</div>
			{/if}
		</div>
	{/each}
	<Button
		variant="ghost"
		class="flex w-full justify-start rounded-t-none transition-colors duration-100 ease-in hover:bg-destructive hover:text-destructive-foreground"
		onclick={() => (showDeleteConfirmation = true)}
		size="sm"
	>
		<Icon icon="solar:trash-bin-2-bold" class="mr-2 size-5" />
		Delete Issue
	</Button>
</ContextMenu>

<DeleteModal
	onConfirm={() => {
		deleteIssue(issue.issue.id);
		showDeleteConfirmation = false;
	}}
	bind:isOpen={showDeleteConfirmation}
	title={`Are you sure you want to delete the issue ${org.custom_id}-${issue.issue.number}`}
	description="This action cannot be undone."
/>

<IssuePannel
	{addComment}
	{deleteIssue}
	{isCreatingIssue}
	{onCreateSubIssue}
	bind:isOpen={isOpenPanel}
	{issue}
	{org}
	onClose={() => (isOpenPanel = false)}
	{updateIssue}
/>

<style>
	[draggable='true'] {
		cursor: grab;
	}

	[draggable='true']:active {
		cursor: grabbing;
	}

	[draggable='true']::after {
		display: none !important;
	}

	.dragging {
		cursor: grabbing !important;
	}
</style>
