<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let faqSection = $state<HTMLElement | null>(null);
	let isInView = $state(false);
	let openIndex = $state<number | null>(null);
	let animatingIndex = $state<number | null>(null);
	let contentHeights = $state<{ [key: number]: number }>({});

	const faqs = [
		{
			question: 'How does the free trial work?',
			answer:
				'Start with a 14-day free trial, no credit card required. Get full access to all features and set up your team workspace. If you love it (we think you will), upgrade to a paid plan at any time.'
		},
		{
			question: 'Can I import data from other tools?',
			answer:
				"Yes! We support importing from popular tools like Jira, Trello, and Asana. Our import wizard makes it easy to migrate your existing projects and maintain your team's workflow."
		},
		{
			question: 'What kind of support do you offer?',
			answer:
				'All plans include community support. Pro and Enterprise plans include priority email support with 24-hour response time. Enterprise plans also get dedicated support with custom SLAs.'
		},
		{
			question: 'Is there a limit on team members?',
			answer:
				'Starter plans support up to 5 team members, Pro plans up to 20, and Enterprise plans have unlimited team members. You can add or remove team members at any time.'
		},
		{
			question: 'How secure is my data?',
			answer:
				'We take security seriously. All data is encrypted in transit and at rest. We use AWS infrastructure, implement regular security audits, and are SOC 2 Type II compliant.'
		},
		{
			question: 'Can I cancel my subscription anytime?',
			answer:
				'Yes, you can cancel your subscription at any time. We offer prorated refunds for annual plans. Your data will be available for export for 30 days after cancellation.'
		}
	];

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

			if (faqSection) {
				observer.observe(faqSection);
			}
		}

		return () => {
			if (faqSection) {
				observer.unobserve(faqSection);
			}
		};
	});

	function toggleFaq(index: number) {
		animatingIndex = index;
		if (openIndex === index) {
			openIndex = null;
		} else {
			openIndex = index;
		}
		setTimeout(() => {
			animatingIndex = null;
		}, 300);
	}
</script>

<section bind:this={faqSection} class="relative py-24 sm:py-32">
	<div class="container relative">
		<div class="mx-auto max-w-2xl text-center">
			<h2
				class="text-3xl font-bold tracking-tight transition-all duration-700 sm:text-4xl {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Frequently asked questions
			</h2>
			<p
				class="mt-6 text-lg leading-8 text-muted-foreground transition-all delay-100 duration-700 {isInView
					? 'translate-y-0 opacity-100'
					: 'translate-y-4 opacity-0'}"
			>
				Can't find what you're looking for? <button class="text-primary hover:underline"
					>Contact our support team</button
				>
			</p>
		</div>

		<div class="mx-auto mt-16 max-w-3xl">
			{#each faqs as faq, i}
				<div
					class="transition-all duration-700 {isInView
						? 'translate-y-0 opacity-100'
						: 'translate-y-4 opacity-0'}"
					style="transition-delay: {100 + i * 100}ms"
				>
					<button
						class="group flex w-full items-center justify-between border-b border-border py-6 text-left transition-colors hover:text-primary"
						onclick={() => toggleFaq(i)}
					>
						<span class="text-lg font-medium">{faq.question}</span>
						<Icon
							icon="material-symbols:keyboard-arrow-down-rounded"
							class="h-6 w-6 transition-transform duration-200 {openIndex === i
								? 'rotate-180'
								: ''}"
						/>
					</button>
					<div
						class="overflow-hidden transition-all duration-300 ease-out"
						style="max-height: {openIndex === i ? '200px' : '0px'}; opacity: {openIndex === i
							? '1'
							: '0'}"
					>
						<p class="py-6 text-muted-foreground">
							{faq.answer}
						</p>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>
