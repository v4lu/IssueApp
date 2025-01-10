<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui';

	const steps = [
		{
			icon: 'material-symbols:input-rounded',
			title: 'Connect Your Tools',
			description: 'Seamlessly integrate with your existing workflow tools and data sources.',
			color: 'blue'
		},
		{
			icon: 'material-symbols:auto-fix-high',
			title: 'Automate Workflows',
			description: 'Build powerful automation rules with our visual workflow builder.',
			color: 'purple'
		},
		{
			icon: 'material-symbols:insights',
			title: 'Track Progress',
			description: 'Monitor performance and get insights with real-time analytics.',
			color: 'green'
		}
	];

	let processSection = $state<HTMLElement | null>(null);
	let isInView = $state(false);

	onMount(() => {
		let observer: IntersectionObserver;
		if (browser) {
			observer = new IntersectionObserver(
				(entries) => {
					entries.forEach((entry) => {
						if (entry.isIntersecting) {
							isInView = true;
							observer.unobserve(entry.target);
						}
					});
				},
				{ threshold: 0.1 }
			);

			if (processSection) {
				observer.observe(processSection);
			}
		}

		return () => {
			if (processSection) {
				observer.unobserve(processSection);
			}
		};
	});
</script>

<section bind:this={processSection} class="relative py-24 sm:py-32">
	<div class="container relative">
		<!-- Connected dots background -->
		<div class="absolute inset-0 -z-10">
			<div
				class="absolute inset-0 opacity-[0.03]"
				style="background-image: radial-gradient(#000 1px, transparent 1px); background-size: 32px 32px;"
			></div>
		</div>

		<!-- Section Header -->
		<div class="mx-auto max-w-2xl text-center">
			<h2
				class="text-3xl font-bold tracking-tight transition-all duration-700 sm:text-4xl {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				How It Works
			</h2>
			<p
				class="mt-6 text-lg leading-8 text-muted-foreground transition-all delay-100 duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Get started in minutes with our simple three-step process
			</p>
		</div>

		<!-- Process Steps -->
		<div class="relative mt-16">
			<!-- Connecting Line -->

			<div class="relative grid gap-8 lg:grid-cols-3">
				{#each steps as step, i}
					<div
						class="group relative transition-all duration-700 {isInView
							? 'translate-y-0 opacity-100'
							: 'translate-y-8 opacity-0'}"
						style="transition-delay: {200 + i * 100}ms"
					>
						<!-- Card -->
						<div
							class="relative overflow-hidden rounded-2xl border border-border bg-background/50 p-8 backdrop-blur-sm"
						>
							<!-- Gradient Background -->
							<div
								class="absolute inset-0 bg-gradient-to-br opacity-[0.08] transition-opacity duration-300 group-hover:opacity-20"
								style={`background-image: linear-gradient(to bottom right, var(--color-${step.color}-500), var(--color-${step.color}-300));`}
							></div>

							<!-- Icon -->
							<div
								class="mb-6 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10"
							>
								<Icon
									icon={step.icon}
									class="h-6 w-6 text-primary transition-transform duration-300 group-hover:scale-110"
								/>
							</div>

							<!-- Content -->
							<h3 class="text-xl font-semibold">{step.title}</h3>
							<p class="mt-4 text-muted-foreground">{step.description}</p>

							<!-- Interactive Elements -->
							<div class="mt-6 flex items-center space-x-4">
								<div class="h-1.5 w-1.5 rounded-full bg-primary/40"></div>
								<div class="h-1.5 w-1.5 rounded-full bg-primary/40"></div>
								<div class="h-1.5 w-1.5 rounded-full bg-primary/40"></div>
							</div>

							<!-- Hover Effect -->
							<div
								class="absolute inset-0 rounded-2xl bg-gradient-to-br from-primary/5 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
							></div>
						</div>
					</div>
				{/each}
			</div>

			<!-- Interactive Demo -->
			<div
				class="mt-16 overflow-hidden rounded-3xl border border-border bg-background/50 p-8 transition-all duration-1000 lg:p-16 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-12 opacity-0'}"
				style="transition-delay: 600ms"
			>
				<div class="grid gap-8 lg:grid-cols-2">
					<div class="md:flex md:flex-col md:justify-center">
						<h3 class="text-2xl font-bold sm:text-3xl">See it in action</h3>
						<p class="mt-4 text-lg text-muted-foreground">
							Watch how our platform transforms your workflow in real-time.
						</p>
						<Button class="mt-4 w-fit">
							<Icon icon="material-symbols:play-circle" class="h-5 w-5" />
							<span>Watch Demo</span>
						</Button>
					</div>

					<!-- Demo Preview -->
					<div class="relative aspect-video rounded-xl border border-border bg-background/80">
						<div class="absolute inset-0 flex items-center justify-center">
							<Icon icon="material-symbols:play-circle" class="h-16 w-16 text-primary/80" />
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>
