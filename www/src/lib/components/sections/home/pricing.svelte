<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let isAnnual = $state(true);
	let animatingPrice = $state(false);

	const plans = $derived([
		{
			name: 'Starter',
			price: isAnnual ? 29 : 39,
			description: 'Perfect for small teams getting started',
			features: ['Up to 5 team members', '5GB storage', 'Basic automation', 'Community support'],
			gradient: 'from-blue-500/20 via-blue-500/10 to-transparent',
			popular: false
		},
		{
			name: 'Pro',
			price: isAnnual ? 79 : 89,
			description: 'Best for growing teams and organizations',
			features: [
				'Up to 20 team members',
				'Unlimited storage',
				'Advanced automation',
				'Priority support',
				'Custom integrations',
				'Analytics dashboard'
			],
			gradient: 'from-primary/20 via-primary/10 to-transparent',
			popular: true
		},
		{
			name: 'Enterprise',
			price: 'Custom',
			description: 'For large-scale organizations',
			features: [
				'Unlimited team members',
				'Advanced security',
				'Custom workflows',
				'Dedicated support',
				'SLA guarantee',
				'Custom training'
			],
			gradient: 'from-purple-500/20 via-purple-500/10 to-transparent',
			popular: false
		}
	]);

	let pricingSection = $state<HTMLElement | null>(null);
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

			if (pricingSection) {
				observer.observe(pricingSection);
			}
		}

		return () => {
			if (pricingSection) {
				observer.unobserve(pricingSection);
			}
		};
	});

	function togglePlan(val: boolean) {
		animatingPrice = true;
		setTimeout(() => {
			isAnnual = val;
			setTimeout(() => {
				animatingPrice = false;
			}, 50);
		}, 150);
	}
</script>

<section bind:this={pricingSection} class="relative py-24 sm:py-32">
	<div class="container relative">
		<div class="mx-auto max-w-2xl text-center">
			<h2
				class="text-3xl font-bold tracking-tight transition-all duration-700 sm:text-4xl {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Simple, transparent pricing
			</h2>
			<p
				class="mt-6 text-lg leading-8 text-muted-foreground transition-all delay-100 duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Choose the perfect plan for your team's needs
			</p>

			<div
				class="mt-8 inline-flex items-center rounded-full border border-border bg-background/50 p-1 backdrop-blur-sm transition-all delay-200 duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				<button
					class="rounded-full px-4 py-2 text-sm transition-colors {isAnnual
						? 'bg-primary text-primary-foreground'
						: 'hover:text-primary'}"
					onclick={() => togglePlan(true)}
				>
					Annual
					<span class="ml-1 text-xs opacity-75">(-20%)</span>
				</button>
				<button
					class="rounded-full px-4 py-2 text-sm transition-colors {!isAnnual
						? 'bg-primary text-primary-foreground'
						: 'hover:text-primary'}"
					onclick={() => togglePlan(false)}
				>
					Monthly
				</button>
			</div>
		</div>

		<div class="mt-16 grid gap-8 lg:grid-cols-3">
			{#each plans as plan, i}
				<div
					class="group relative transition-all duration-700 {isInView
						? 'translate-y-0 opacity-100'
						: 'translate-y-8 opacity-0'}"
					style="transition-delay: {200 + i * 100}ms"
				>
					<div
						class="relative overflow-hidden rounded-2xl border border-border bg-background/50 p-8 backdrop-blur-sm"
					>
						{#if plan.popular}
							<div
								class="absolute right-4 top-4 rounded-full bg-primary/10 px-3 py-1 text-xs font-medium text-primary"
							>
								Most Popular
							</div>
						{/if}

						<div
							class="absolute inset-0 bg-gradient-to-br {plan.gradient} opacity-[0.08] transition-opacity duration-300 group-hover:opacity-20"
						></div>

						<div class="relative">
							<h3 class="text-xl font-semibold">{plan.name}</h3>
							<p class="mt-2 text-sm text-muted-foreground">{plan.description}</p>

							<div class="mt-6 h-[50px]">
								<div class="flex items-baseline">
									{#if typeof plan.price === 'number'}
										<span
											class="text-4xl font-bold transition-all duration-300 {animatingPrice
												? 'scale-95 opacity-0'
												: 'scale-100 opacity-100'}"
										>
											${plan.price}
										</span>
										<span
											class="ml-1 text-muted-foreground transition-all duration-300 {animatingPrice
												? 'translate-y-1 opacity-0'
												: 'translate-y-0 opacity-100'}"
										>
											/mo
										</span>
									{:else}
										<span class="text-4xl font-bold">{plan.price}</span>
									{/if}
								</div>
								{#if typeof plan.price === 'number'}
									<div
										class="absolute left-0 right-0 transition-all duration-500 {isAnnual
											? 'translate-y-0 opacity-100'
											: 'translate-y-4 opacity-0'}"
									>
										{#if isAnnual}
											<p class="mt-1 text-sm text-muted-foreground">
												Billed annually (${plan.price * 12}/year)
											</p>
										{/if}
									</div>
								{/if}
							</div>

							<ul class="mt-8 space-y-4">
								{#each plan.features as feature}
									<li class="flex items-center text-sm">
										<Icon
											icon="material-symbols:check-circle-outline"
											class="mr-2 h-5 w-5 text-primary"
										/>
										{feature}
									</li>
								{/each}
							</ul>

							<button
								class="mt-8 w-full rounded-lg border border-primary bg-primary px-4 py-2 text-sm font-semibold text-primary-foreground transition-colors hover:bg-primary/90 {plan.popular
									? ''
									: 'border-primary/20 bg-primary/10 text-primary text-slate-300 hover:bg-primary/20'}"
							>
								{plan.name === 'Enterprise' ? 'Contact Sales' : 'Get Started'}
							</button>
						</div>

						<div
							class="absolute inset-0 rounded-2xl bg-gradient-to-br from-primary/5 via-transparent to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100"
						></div>
					</div>
				</div>
			{/each}
		</div>

		<div
			class="mt-16 text-center transition-all duration-1000 {isInView
				? 'translate-y-0 opacity-100'
				: 'translate-y-12 opacity-0'}"
			style="transition-delay: 600ms"
		>
			<p class="text-muted-foreground">
				Have questions? Check out our <button class="text-primary hover:underline">FAQ</button> or
				<button class="text-primary hover:underline">contact us</button>.
			</p>
		</div>
	</div>
</section>

<style>
	@keyframes pulse-scale {
		0% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.05);
		}
		100% {
			transform: scale(1);
		}
	}

	.price-animate {
		animation: pulse-scale 0.3s ease-in-out;
	}
</style>
