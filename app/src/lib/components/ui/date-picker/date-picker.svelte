<script lang="ts">
	import {
		addDays,
		addMonths,
		eachDayOfInterval,
		endOfMonth,
		format,
		getDay,
		isBefore,
		isSameDay,
		isSameMonth,
		startOfDay,
		startOfMonth,
		subDays,
		subMonths
	} from 'date-fns';

	type Props = {
		selected: Date | null;
		selectDate: (date: Date | null) => void;
	};

	let { selected, selectDate }: Props = $props();
	let currentMonth = $state(startOfMonth(new Date()));
	let today = $state(startOfDay(new Date()));

	function nextMonth() {
		currentMonth = addMonths(currentMonth, 1);
	}

	function prevMonth() {
		currentMonth = subMonths(currentMonth, 1);
	}

	const start = $derived(startOfMonth(currentMonth));
	const end = $derived(endOfMonth(currentMonth));
	const startDay = $derived(getDay(start));
	const paddingDays = $derived(startDay === 0 ? 6 : startDay - 1);
	const paddingStart = $derived(subDays(start, paddingDays));
	const paddingEnd = $derived(addDays(end, 42 - (paddingDays + end.getDate())));
	let calendarDays = $derived(eachDayOfInterval({ start: paddingStart, end: paddingEnd }));

	function handleDateClick(day: Date) {
		if (selected && isSameDay(day, selected)) {
			selectDate(null);
		} else {
			selectDate(day);
		}
	}
</script>

<div class="p-4">
	<div class="mb-4 flex justify-between">
		<button onclick={prevMonth}>&lt;</button>
		<span>{format(currentMonth, 'MMMM yyyy')}</span>
		<button onclick={nextMonth}>&gt;</button>
	</div>
	<div class="grid grid-cols-7 gap-1">
		{#each ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'] as day}
			<div class="text-center text-sm font-bold">{day}</div>
		{/each}
		{#each calendarDays as day}
			<button
				class="p-2 text-center
        {isSameMonth(day, currentMonth) ? '' : 'text-muted-foreground'}
        {isBefore(day, today) ? 'text-muted-foreground/50' : ''}
        {selected && isSameDay(day, selected)
					? 'rounded-md bg-primary text-primary-foreground'
					: ''}"
				onclick={() => handleDateClick(day)}
			>
				{format(day, 'd')}
			</button>
		{/each}
	</div>
</div>
