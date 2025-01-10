<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fly } from 'svelte/transition';
	import { expoInOut } from 'svelte/easing';
	import { Icons } from '../icons';
	import { Badge } from '../ui/badge';
	import { Button } from '../ui/button';
	import { Dropdown } from '../ui/dropdown';
	import { Editor } from '../ui/editor';
	import { Input } from '../ui/input';
	import { DatePicker } from '../ui/date-picker';
	import CreateIssue from '../modals/create-issue.svelte';
	import { Avatar } from '../ui/avatar';
	import { IssueRowMiniCard } from '.';
	import type { IssueRequest, IssueResponse, IssueUpdate } from '$lib/types/issue.type';
	import { priorityOrder, statusOrder } from '$lib/api/issue.svelte';
	import {
		clickOutside,
		cn,
		getIcon,
		getPriorityColor,
		getStatusColor,
		getStatusName,
		monthAndDay
	} from '$lib';
	import type { Org } from '$lib/types/org.type';
	import type { CommentRequest } from '$lib/types/comment.type';

	type Props = {
		issue: IssueResponse;
		onClose: () => void;
		updateIssue: (id: string, issue: IssueUpdate) => Promise<void>;
		isOpen: boolean;
		org: Org;
		isCreatingIssue: boolean;
		onCreateSubIssue: (issue: IssueRequest) => Promise<void>;
		deleteIssue: (id: string) => Promise<void>;
		addComment: (req: CommentRequest) => Promise<void>;
	};

	let {
		issue,
		onClose,
		updateIssue,
		isOpen = $bindable(),
		org,
		onCreateSubIssue,
		isCreatingIssue,
		deleteIssue,
		addComment
	}: Props = $props();

	let statusDropdownOpen = $state(false);
	let priorityDropdownOpen = $state(false);
	let isDatePickerOpen = $state(false);
	let createSubIssue = $state(false);

	let title = $state(issue.issue.title);
	let description = $state(issue.issue.description);
	let status = $state(issue.issue.status);
	let priority = $state(issue.issue.priority);
	let due_date = $state(issue.issue.due_date);

	let comment = $state({});

	function parseJson(description: string | undefined): object {
		if (!description) return {};
		try {
			return JSON.parse(description);
		} catch (error) {
			console.error('Error parsing description:', error);
			return {
				type: 'doc',
				content: [{ type: 'paragraph', content: [{ type: 'text', text: description }] }]
			};
		}
	}

	function handleDescriptionUpdate(content: object) {
		description = JSON.stringify(content);
	}

	function handleCommentUpdate(content: object) {
		comment = JSON.stringify(content);
	}

	function handleTitlechange(e: Event) {
		const target = e.target as HTMLInputElement;
		title = target.value;
	}

	function handleDateSelect(date: Date | null) {
		due_date = date;
		isDatePickerOpen = false;
	}

	function handleUpdateIssue() {
		const payload: IssueUpdate = {
			title,
			description,
			status,
			priority,
			...(due_date !== issue.issue.due_date && {
				due_date,
				remove_due_date: due_date === null
			})
		};

		updateIssue(issue.issue.id, payload);
	}

	async function handleSubIssue(i: IssueRequest) {
		const payload: IssueRequest = {
			...i,
			parent_id: issue.issue.id
		};
		void onCreateSubIssue(payload);
		isOpen = true;
	}
</script>

{#if isOpen}
	<div
		use:clickOutside={onClose}
		transition:fly={{ x: 100, duration: 400, easing: expoInOut }}
		class="scrollbar fixed inset-y-0 right-0 top-0 z-[50] max-w-[40rem] overflow-y-auto border-l border-border bg-card shadow-lg"
	>
		<div
			class="sticky top-0 z-10 h-[65px] border-b border-border bg-background/95 px-4 py-3 backdrop-blur supports-[backdrop-filter]:bg-background/60"
		>
			<div class="flex h-full items-center justify-between">
				<div class="flex items-center justify-center gap-4">
					<Badge>
						{`${org.custom_id}-${issue.issue.number}`}
					</Badge>
					<h2 class="text-xl font-semibold tracking-tight">
						{issue ? 'Edit Issue' : 'Loading Issue...'}
					</h2>
				</div>
				<Button variant="ghost" size="icon" onclick={onClose} class="hover:bg-muted">
					<Icon icon="lucide:x" class="size-5" />
				</Button>
			</div>
		</div>

		<div class="relative flex-1">
			<Button size="sm" class="absolute right-4 top-4" onclick={() => (createSubIssue = true)}>
				Create Sub Issue
			</Button>
			<div class="space-y-8 p-6">
				<div class="space-y-2">
					<p class="text-sm font-medium text-muted-foreground">Title</p>
					<Input
						oninput={handleTitlechange}
						variant="empty"
						value={title}
						class="border-0 bg-transparent px-0 text-xl font-medium focus-visible:ring-0"
					/>
				</div>

				<div class="space-y-2">
					<p class="text-sm font-medium text-muted-foreground">Due Date</p>
					<Dropdown
						triggerIconPosition="left"
						triggerIcon="solar:calendar-outline"
						triggerIconClass="mr-2"
						triggerClass={cn('w-fit min-w-[0] px-3 py-2 text-sm border rounded-md hover:bg-muted')}
						triggerText={monthAndDay(due_date) === '-'
							? 'Select a due date'
							: monthAndDay(due_date)}
						bind:isOpen={isDatePickerOpen}
						class="w-fit"
					>
						<div class="flex flex-col">
							<DatePicker selected={due_date && new Date(due_date)} selectDate={handleDateSelect} />
							{#if due_date}
								<button
									class="flex w-full items-center gap-2 border-t border-border p-2 text-sm text-destructive hover:bg-muted"
									onclick={() => handleDateSelect(null)}
								>
									<Icon icon="solar:calendar-remove-bold" class="size-4" />
									<span>Remove due date</span>
								</button>
							{/if}
						</div>
					</Dropdown>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-2">
						<p class="text-sm font-medium text-muted-foreground">Status</p>
						<Dropdown
							bind:isOpen={statusDropdownOpen}
							triggerText={getStatusName(status)}
							downArrowIcon
							CustomIcon={Icons.status[status]}
							customIconClass={cn(getStatusColor(status), 'size-4')}
							triggerIconPosition="left"
							triggerClass="w-full justify-between px-3 py-2 text-sm border rounded-md hover:bg-muted"
						>
							<div class="p-1">
								{#each statusOrder as s}
									{@const Icon = Icons.status[s]}
									<button
										class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted"
										onclick={() => {
											status = s;
											statusDropdownOpen = false;
										}}
									>
										<Icon class={cn('size-4', getStatusColor(s))} />
										<span>{getStatusName(s)}</span>
									</button>
								{/each}
							</div>
						</Dropdown>
					</div>

					<div class="space-y-2">
						<p class="text-sm font-medium text-muted-foreground">Priority</p>
						<Dropdown
							bind:isOpen={priorityDropdownOpen}
							triggerText={priority}
							downArrowIcon
							CustomIcon={Icons.priority[priority]}
							customIconClass={cn(getPriorityColor(priority), 'size-4')}
							triggerIconPosition="left"
							triggerClass="w-full justify-between px-3 py-2 text-sm border rounded-md hover:bg-muted"
						>
							<div class="p-1">
								{#each priorityOrder as p}
									{@const Icon = Icons.priority[p]}
									<button
										class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-sm hover:bg-muted"
										onclick={() => {
											priority = p;
											priorityDropdownOpen = false;
										}}
									>
										<Icon class={cn('size-4', getPriorityColor(p))} />
										<span>{p}</span>
									</button>
								{/each}
							</div>
						</Dropdown>
					</div>
				</div>

				<div class="space-y-2">
					<p class="text-sm font-medium text-muted-foreground">Description</p>
					<Editor
						content={parseJson(description)}
						update={handleDescriptionUpdate}
						placeholder="Add a description..."
					/>
				</div>
			</div>
		</div>

		<div class="flex w-full items-center justify-end px-4">
			<Button onclick={handleUpdateIssue}>Save Changes</Button>
		</div>
		<div class="mt-4">
			<h2 class="px-4 text-xl font-medium">Comments</h2>
			<Editor
				placeholder="Add a comment..."
				content={comment}
				class="0 scrollbar-hidden h-32 max-h-32 min-h-0 overflow-y-auto"
				update={handleCommentUpdate}
			/>
			<div class="mt-4 flex justify-end px-4">
				<Button
					size="sm"
					class="mt-2"
					onclick={() => {
						addComment({
							comment_owner_id: issue.issue.id,
							comment_type: 'Issue',
							content: JSON.stringify(comment)
						});
						comment = {};
					}}
				>
					Add Comment
				</Button>
			</div>
			<div class="mt-4 space-y-6 border-t border-border pt-4">
				{#if issue.comments}
					{#each issue.comments as comment}
						<div class="rounded-lg bg-card p-4 shadow-sm">
							<div class="mb-3 flex items-center gap-4">
								<Avatar user={comment.creator} class="size-12" />
								<div class="flex flex-col">
									<span class="font-medium">
										{comment.creator.username}
									</span>
									<span class="text-xs text-muted-foreground">
										{monthAndDay(comment.comment.created_at)}
									</span>
								</div>
							</div>

							<div>
								{comment.comment.id}
							</div>
						</div>
					{/each}
				{:else}
					<div class="py-6 text-center text-muted-foreground">No comments yet</div>
				{/if}
			</div>
		</div>

		<div class="mb-12 mt-4">
			<h2 class="px-4 text-xl font-medium">Sub issues</h2>
			<div class="mt-4 grid border-t border-border">
				{#if issue.sub_issues}
					{#each issue.sub_issues as subIssue}
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
				{/if}
			</div>
		</div>
	</div>
{/if}

<CreateIssue
	bind:isOpen={createSubIssue}
	onClose={() => (createSubIssue = false)}
	{isCreatingIssue}
	createIssue={handleSubIssue}
/>
