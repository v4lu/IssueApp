<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { FeatureBigCard } from '$lib/components/card';

	const features = [
		{
			icon: 'material-symbols:speed-outline',
			title: 'Lightning Fast',
			description: 'Built for speed with instant updates and real-time collaboration.',
			gradient: 'from-blue-500/20 via-blue-500/10 to-transparent'
		},
		{
			icon: 'material-symbols:auto-awesome',
			title: 'AI-Powered',
			description: 'Smart automation and intelligent suggestions to boost your productivity.',
			gradient: 'from-purple-500/20 via-purple-500/10 to-transparent'
		},
		{
			icon: 'material-symbols:integration-instructions',
			title: 'Seamless Integration',
			description: 'Connects with your favorite tools right out of the box.',
			gradient: 'from-green-500/20 via-green-500/10 to-transparent'
		},
		{
			icon: 'material-symbols:lock-outline',
			title: 'Enterprise Security',
			description: 'Bank-grade security with advanced permissions and audit logs.',
			gradient: 'from-orange-500/20 via-orange-500/10 to-transparent'
		}
	];

	let featuresSection = $state<HTMLElement | null>(null);
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

			if (featuresSection) {
				observer.observe(featuresSection);
			}
		}

		return () => {
			if (featuresSection) {
				observer.unobserve(featuresSection);
			}
		};
	});
</script>

<section class="relative py-24 sm:py-32" bind:this={featuresSection}>
	<div class="container">
		<div class="mx-auto max-w-2xl text-center">
			<h2
				class="text-3xl font-bold tracking-tight transition-all duration-700 sm:text-4xl {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Everything you need to ship faster
			</h2>
			<p
				class="mt-6 text-lg leading-8 text-muted-foreground transition-all delay-100 duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Powerful features to help your team stay organized, focused, and ahead of deadlines.
			</p>
		</div>

		<div class="mx-auto mt-16 max-w-7xl">
			<div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-2">
				{#each features as feature, i}
					<div
						class="relative overflow-hidden rounded-2xl border border-border bg-background/50 p-8 backdrop-blur-sm transition-all duration-700 {isInView
							? 'translate-y-0 opacity-100'
							: 'translate-y-8 opacity-0'}"
						style="transition-delay: {200 + i * 100}ms"
					>
						<div
							class="pointer-events-none absolute inset-0 bg-gradient-to-br {feature.gradient} opacity-20"
						></div>

						<div
							class="mb-6 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10 {isInView
								? 'animate-in-spin'
								: ''}"
						>
							<Icon icon={feature.icon} class="h-6 w-6 text-primary" />
						</div>

						<div class="relative">
							<h3 class="text-xl font-semibold">{feature.title}</h3>
							<p class="mt-4 text-muted-foreground">
								{feature.description}
							</p>
						</div>

						<div
							class="absolute inset-0 rounded-2xl bg-gradient-to-br from-primary/10 via-transparent to-transparent opacity-0 transition-opacity duration-300 hover:opacity-100"
						></div>
					</div>
				{/each}
			</div>
		</div>

		<FeatureBigCard {isInView} />
	</div>
</section>

<style>
	/* Add these new animations */
	@keyframes gradient {
		0%,
		100% {
			transform: rotate(0deg) scale(1);
		}
		50% {
			transform: rotate(180deg) scale(1.1);
		}
	}

	@keyframes floating {
		0%,
		100% {
			transform: translateY(0) translateX(0);
			opacity: 0;
		}
		50% {
			transform: translateY(-100px) translateX(100px);
			opacity: 1;
		}
	}

	@keyframes particle {
		0% {
			transform: scale(0) translateY(0);
			opacity: 0;
		}
		50% {
			opacity: 1;
		}
		100% {
			transform: scale(1.5) translateY(-100px);
			opacity: 0;
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

	.animate-gradient {
		animation: gradient 8s ease infinite;
	}

	.animate-floating {
		animation: floating 4s ease-in-out infinite;
	}

	.animate-particle {
		animation: particle 3s ease-out infinite;
	}

	.animate-scan {
		animation: scan 2s linear infinite;
	}

	.animate-in-spin {
		animation: spin-in 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards;
	}

	@keyframes spin-in {
		0% {
			transform: rotate(-180deg) scale(0.6);
			opacity: 0;
		}
		100% {
			transform: rotate(0deg) scale(1);
			opacity: 1;
		}
	}
</style>
