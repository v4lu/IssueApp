<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let footerSection = $state<HTMLElement | null>(null);
	let isInView = $state(false);

	const navigation = {
		product: [
			{ name: 'Features', href: '#features' },
			{ name: 'Pricing', href: '#pricing' },
			{ name: 'Security', href: '#security' },
			{ name: 'Changelog', href: '#changelog' },
			{ name: 'Documentation', href: '#docs' }
		],
		company: [
			{ name: 'About', href: '#about' },
			{ name: 'Blog', href: '#blog' },
			{ name: 'Careers', href: '#careers' },
			{ name: 'Press', href: '#press' },
			{ name: 'Partners', href: '#partners' }
		],
		support: [
			{ name: 'Help Center', href: '#help' },
			{ name: 'Contact Us', href: '#contact' },
			{ name: 'Status', href: '#status' },
			{ name: 'API Status', href: '#api-status' },
			{ name: 'Community', href: '#community' }
		],
		legal: [
			{ name: 'Privacy', href: '#privacy' },
			{ name: 'Terms', href: '#terms' },
			{ name: 'Cookie Policy', href: '#cookies' },
			{ name: 'License', href: '#license' }
		],
		social: [
			{
				name: 'X',
				href: '#',
				icon: 'fa6-brands:x-twitter'
			},
			{
				name: 'GitHub',
				href: '#',
				icon: 'mdi:github'
			},
			{
				name: 'Discord',
				href: '#',
				icon: 'ic:baseline-discord'
			}
		]
	};

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

			if (footerSection) {
				observer.observe(footerSection);
			}
		}

		return () => {
			if (footerSection) {
				observer.unobserve(footerSection);
			}
		};
	});
</script>

<footer bind:this={footerSection} class="border-trelative mt-32 sm:mt-40">
	<div class="border-t border-border">
		<div class="container pb-8 pt-16 xl:grid xl:grid-cols-3 xl:gap-8">
			<div
				class="space-y-8 transition-all duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				<div class="flex items-center space-x-2">
					<Icon icon="lucide:box" class="h-8 w-8 text-primary" />
					<span class="text-2xl font-bold">TSMG</span>
				</div>
				<p class="text-sm leading-6 text-muted-foreground">
					Making project management more efficient, collaborative, and enjoyable for teams
					worldwide.
				</p>
				<div class="flex space-x-6">
					{#each navigation.social as item}
						<a href={item.href} class="text-muted-foreground transition-colors hover:text-primary">
							<span class="sr-only">{item.name}</span>
							<Icon icon={item.icon} class="h-6 w-6" />
						</a>
					{/each}
				</div>
			</div>

			<!-- Navigation Grid -->
			<div
				class="mt-16 grid grid-cols-2 gap-8 transition-all delay-200 duration-700 xl:col-span-2 xl:mt-0 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				<div class="md:grid md:grid-cols-2 md:gap-8">
					<div>
						<h3 class="text-sm font-semibold leading-6">Product</h3>
						<ul class="mt-6 space-y-4">
							{#each navigation.product as item}
								<li>
									<a
										href={item.href}
										class="text-sm leading-6 text-muted-foreground transition-colors hover:text-primary"
									>
										{item.name}
									</a>
								</li>
							{/each}
						</ul>
					</div>
					<div class="mt-10 md:mt-0">
						<h3 class="text-sm font-semibold leading-6">Company</h3>
						<ul class="mt-6 space-y-4">
							{#each navigation.company as item}
								<li>
									<a
										href={item.href}
										class="text-sm leading-6 text-muted-foreground transition-colors hover:text-primary"
									>
										{item.name}
									</a>
								</li>
							{/each}
						</ul>
					</div>
				</div>
				<div class="md:grid md:grid-cols-2 md:gap-8">
					<div>
						<h3 class="text-sm font-semibold leading-6">Support</h3>
						<ul class="mt-6 space-y-4">
							{#each navigation.support as item}
								<li>
									<a
										href={item.href}
										class="text-sm leading-6 text-muted-foreground transition-colors hover:text-primary"
									>
										{item.name}
									</a>
								</li>
							{/each}
						</ul>
					</div>
					<div class="mt-10 md:mt-0">
						<h3 class="text-sm font-semibold leading-6">Legal</h3>
						<ul class="mt-6 space-y-4">
							{#each navigation.legal as item}
								<li>
									<a
										href={item.href}
										class="text-sm leading-6 text-muted-foreground transition-colors hover:text-primary"
									>
										{item.name}
									</a>
								</li>
							{/each}
						</ul>
					</div>
				</div>
			</div>
		</div>

		<div
			class=" mt-4 border-t border-border transition-all delay-300 duration-700 {isInView
				? 'translate-y-0 opacity-100'
				: 'translate-y-4 opacity-0'}"
		>
			<div class="container flex flex-col items-center justify-between gap-4 py-4 sm:flex-row">
				<p class="text-sm leading-6 text-muted-foreground">
					&copy; {new Date().getFullYear()} TSMG, Inc. All rights reserved.
				</p>
				<div class="flex items-center space-x-1 text-sm text-muted-foreground">
					<Icon icon="material-symbols:language" class="h-5 w-5" />
					<select
						class="cursor-pointer border-none bg-transparent transition-colors hover:text-primary focus:ring-0"
					>
						<option value="en-US">English (US)</option>
						<option value="es">Español</option>
						<option value="fr">Français</option>
						<option value="de">Deutsch</option>
					</select>
				</div>
			</div>
		</div>
	</div>
</footer>

<style>
	select {
		/* Remove default arrow in Firefox */
		-moz-appearance: none;
		appearance: none;
		/* Remove default arrow in Chrome */
		-webkit-appearance: none;
		appearance: none;
	}

	/* Remove default arrow in IE */
	select::-ms-expand {
		display: none;
	}

	select option {
		@apply bg-background text-foreground;
	}
</style>
