<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Badge } from '../ui/badge';
	import { Button } from '../ui/button';
	import { Dropdown } from '../ui/dropdown';
	import { Editor } from '../ui/editor';
	import { Input } from '../ui/input';
	import { Modal } from '../ui/modal';
	import { Toggle } from '../ui/toggle';
	import { DatePicker } from '../ui/date-picker';
	import { type PriorityArrType, type StatusArrType, priorityArr, statusArr } from '.';
	import type { IssueRequest } from '$lib/types/issue.type';
	import { orgStore } from '$lib/stores/org.store';
	import { getSelectedDateLabel } from '$lib';

	type Props = {
		isOpen: boolean;
		onClose: () => void;
		createIssue: (issue: IssueRequest) => Promise<void>;
		isCreatingIssue: boolean;
	};

	let { isOpen = $bindable(), onClose, createIssue, isCreatingIssue }: Props = $props();

	let createMore = $state(false);

	let isPriorityDropdownOpen = $state(false);
	let selectedPriority = $state<PriorityArrType>(priorityArr[0]);
	let isStatusDropdownOpen = $state(false);
	let selectedStatus = $state<StatusArrType>(statusArr[0]);
	let isDatePickerOpen = $state(false);
	let selectedDate = $state<Date | null>(null);
	let title = $state('');
	let description = $state<object>({});

	function handleDateSelect(date: Date | null) {
		selectedDate = date;
		isDatePickerOpen = false;
	}

	function handleDescriptionUpdate(content: object) {
		description = content;
	}

	async function handleCreateWorkspace() {
		const payload: IssueRequest = {
			title,
			description: JSON.stringify(description),
			status: selectedStatus.IconName,
			priority: selectedPriority.IconName,
			due_date: selectedDate
		};
		await createIssue(payload);
		title = '';
		description = {};
		selectedPriority = priorityArr[0];
		selectedStatus = statusArr[0];
		selectedDate = null;

		if (!createMore) {
			isOpen = false;
		}
	}
</script>

<Modal
	class="grid gap-2 p-0 md:w-[40rem] md:min-w-[40rem] xl:w-[45rem] xl:min-w-[45rem]"
	bind:isOpen
	{onClose}
>
	<article class="flex w-full items-center justify-between px-6 pt-6">
		<div class="flex items-center justify-center gap-2">
			<Badge>
				{$orgStore.custom_id}
			</Badge>
			<span class="text-xs font-bold"> Create Issue </span>
		</div>
	</article>
	<article class="my-4 max-h-[236px] overflow-y-auto px-6">
		<Input bind:value={title} variant="empty" placeholder="What should be done" class="text-xl" />
		<div class="overflow-y-auto">
			<Editor
				content={description}
				update={handleDescriptionUpdate}
				placeholder="Add a description..."
			/>
		</div>
	</article>

	<article class="flex items-center justify-start gap-2 px-6">
		<Dropdown
			triggerClass="px-2 py-1 gap-0 w-fit min-w-fit text-xs font-medium"
			triggerText={selectedPriority.IconName}
			bind:isOpen={isPriorityDropdownOpen}
			CustomIcon={selectedPriority.Icon}
			customIconClass="size-4 mr-1"
			class="top-[2rem] w-fit min-w-fit"
		>
			{#each priorityArr as priority}
				<Button
					variant="ghost"
					size="sm"
					class="flex w-full justify-start px-4 py-0 text-xs font-medium"
					onclick={() => {
						selectedPriority = priority;
						isPriorityDropdownOpen = false;
					}}
				>
					<priority.Icon class="mr-2 size-4" />
					{priority.IconName}
				</Button>
			{/each}
		</Dropdown>
		<Dropdown
			triggerClass="px-2 py-1 gap-0 w-fit min-w-fit text-xs font-medium"
			triggerText={selectedStatus.IconName}
			bind:isOpen={isStatusDropdownOpen}
			CustomIcon={selectedStatus.Icon}
			customIconClass="size-4 mr-1"
			class="top-[2rem] w-fit min-w-fit"
		>
			{#each statusArr as status}
				<Button
					variant="ghost"
					size="sm"
					class="flex w-full justify-start px-4 py-0 text-xs font-medium"
					onclick={() => {
						selectedStatus = status;
						isStatusDropdownOpen = false;
					}}
				>
					<status.Icon class="mr-2 size-4" />
					{status.IconName}
				</Button>
			{/each}
		</Dropdown>
		<Dropdown
			triggerIconPosition="left"
			triggerIcon="solar:calendar-outline"
			triggerIconClass="mr-2"
			triggerClass="px-2 py-1 gap-0 w-fit min-w-fit text-xs font-medium"
			triggerText={selectedDate ? getSelectedDateLabel(selectedDate) : 'Select due date'}
			bind:isOpen={isDatePickerOpen}
			class="top-[2rem] min-w-[20rem]"
		>
			<DatePicker selected={selectedDate} selectDate={handleDateSelect} />
		</Dropdown>
	</article>
	<article class="flex items-center justify-between border-t border-border px-6 py-3">
		<Button size="icon" variant="ghost">
			<Icon icon="lucide:paperclip" class="size-5" />
		</Button>

		<div class="flex items-center justify-center gap-3">
			<Toggle isActive={createMore} toggle={() => (createMore = !createMore)} />
			<Button disabled={isCreatingIssue} onclick={handleCreateWorkspace} size="sm">
				Create Issue
			</Button>
		</div>
	</article>
</Modal>
