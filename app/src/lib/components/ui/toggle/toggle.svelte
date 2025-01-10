<script lang="ts">
	type Props = {
		isActive: boolean;
		toggle: () => void;
	};

	let { isActive, toggle }: Props = $props();
</script>

<div class="flex items-center gap-2">
	<span class="text-xs text-muted-foreground">Create more</span>
	<div
		class="relative h-5 w-10 cursor-pointer rounded-full bg-muted transition-colors duration-200 ease-in-out"
		class:bg-primary={isActive}
		onclick={toggle}
		onkeydown={(e) => e.key === 'Enter' && toggle()}
		role="button"
		tabindex="0"
	>
		<div
			class="absolute left-0.5 top-0.5 h-4 w-4 transform rounded-full bg-white transition-transform duration-200 ease-in-out"
			class:translate-x-5={isActive}
		></div>
	</div>
</div>
