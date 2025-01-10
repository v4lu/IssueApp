<script lang="ts">
	import Icon from '@iconify/svelte';

	type Props = {
		isInView: boolean;
	};

	let { isInView }: Props = $props();
</script>

<div
	class="group relative mx-auto mt-16 overflow-hidden rounded-3xl border border-border bg-background/50 p-8 backdrop-blur-sm transition-all duration-1000 lg:p-16 {isInView
		? 'translate-y-0 opacity-100'
		: 'translate-y-12 opacity-0'}"
	style="transition-delay: 600ms"
>
	<!-- Subtle gradient background -->
	<div class="absolute inset-0 opacity-0 transition-opacity duration-500 group-hover:opacity-100">
		<div
			class="absolute inset-0 animate-pulse bg-gradient-to-tr from-primary/5 via-transparent to-secondary/5"
		></div>

		<!-- Animated lines -->
		<div class="absolute inset-0">
			{#each Array(8) as _, i}
				<div
					class="group-hover:animate-flow absolute h-[1px] w-[200px] bg-primary/20 blur-[1px]"
					style="left: {Math.random() * 100}%; top: {Math.random() * 100}%; animation-delay: {i *
						0.3}s;"
				></div>
			{/each}
		</div>
	</div>

	<!-- Floating particles -->
	<div class="absolute inset-0 opacity-0 transition-opacity duration-500 group-hover:opacity-100">
		{#each Array(15) as _, i}
			<div
				class="absolute h-1 w-1 rounded-full bg-primary/40 group-hover:animate-float"
				style="left: {Math.random() * 100}%; top: {Math.random() * 100}%; animation-delay: {i *
					0.2}s;"
			></div>
		{/each}
	</div>

	<div class="relative grid gap-8 lg:grid-cols-2 lg:gap-16">
		<!-- Content Section -->
		<div class="flex flex-col justify-center">
			<h3
				class="text-2xl font-bold transition-colors duration-300 group-hover:text-primary sm:text-3xl"
			>
				Advanced Workflow Automation
				<div
					class="relative ml-2 inline-flex opacity-0 transition-opacity duration-300 group-hover:opacity-100"
				>
					<span class="absolute inline-flex h-2 w-2 animate-ping rounded-full bg-primary opacity-75"
					></span>
					<span class="relative inline-flex h-2 w-2 rounded-full bg-primary"></span>
				</div>
			</h3>
			<p class="mt-4 text-lg text-muted-foreground">
				Automate repetitive tasks and focus on what matters. Our intelligent workflow engine handles
				the routine while you handle the strategy.
			</p>
			<ul class="mt-8 space-y-4">
				{#each ['Custom Triggers', 'Smart Rules', 'Template Library', 'Integration Hub'] as item, i}
					<li
						class="flex items-center transition-all duration-500 {isInView
							? 'translate-x-0 opacity-100'
							: '-translate-x-4 opacity-0'}"
						style="transition-delay: {800 + i * 100}ms"
					>
						<Icon
							icon="material-symbols:check-circle-outline"
							class="mr-2 h-5 w-5 text-primary transition-transform duration-300 group-hover:scale-110"
						/>
						<span class="transition-colors duration-300 group-hover:text-primary/90">{item}</span>
					</li>
				{/each}
			</ul>
		</div>

		<!-- Mock Interface -->
		<div
			class="relative overflow-hidden rounded-xl border border-border bg-background/80 p-4 transition-all duration-500 group-hover:shadow-xl group-hover:shadow-primary/10"
		>
			<!-- Window Controls -->
			<div class="flex items-center space-x-2 border-b border-border pb-4">
				<div class="h-3 w-3 rounded-full bg-destructive"></div>
				<div class="h-3 w-3 rounded-full bg-primary"></div>
				<div class="h-3 w-3 rounded-full bg-secondary"></div>
			</div>

			<!-- Interface Elements -->
			<div class="mt-4 space-y-3">
				{#each Array(4) as _, i}
					<div
						class="h-8 rounded bg-accent/20 transition-all duration-500 group-hover:bg-accent/30 {isInView
							? 'translate-x-0 opacity-100'
							: 'translate-x-4 opacity-0'}"
						style="transition-delay: {1000 + i * 100}ms"
					></div>
				{/each}
			</div>

			<!-- Scanning effect -->
			<div
				class="absolute inset-0 opacity-0 transition-opacity duration-300 group-hover:opacity-100"
			>
				<div
					class="animate-scan absolute inset-0 bg-gradient-to-r from-transparent via-primary/10 to-transparent"
				></div>
			</div>
		</div>
	</div>
</div>

<style>
	@keyframes flow {
		0% {
			transform: translateX(-100%) translateY(0);
			opacity: 0;
		}
		50% {
			opacity: 1;
		}
		100% {
			transform: translateX(100%) translateY(-20px);
			opacity: 0;
		}
	}

	@keyframes float {
		0%,
		100% {
			transform: translateY(0) scale(1);
			opacity: 0;
		}
		50% {
			transform: translateY(-40px) scale(1.2);
			opacity: 1;
		}
	}

	@keyframes scan {
		0% {
			transform: translateX(-100%);
		}
		100% {
			transform: translateX(100%);
		}
	}

	.animate-scan {
		animation: scan 2s linear infinite;
		width: 70%;
	}
</style>
