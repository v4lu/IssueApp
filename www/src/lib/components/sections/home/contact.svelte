<script lang="ts">
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { Input, inputVariants } from '$lib/components/ui';
	import { cn } from '$lib';

	let contactSection = $state<HTMLElement | null>(null);
	let isInView = $state(false);
	let formStatus = $state<'idle' | 'submitting' | 'success' | 'error'>('idle');

	// Form data
	let formData = $state({
		name: '',
		email: '',
		company: '',
		message: ''
	});

	// Form validation
	let errors = $state({
		name: '',
		email: '',
		message: ''
	});

	function validateForm() {
		let isValid = true;
		errors = {
			name: '',
			email: '',
			message: ''
		};

		if (!formData.name.trim()) {
			errors.name = 'Name is required';
			isValid = false;
		}

		if (!formData.email.trim()) {
			errors.email = 'Email is required';
			isValid = false;
		} else if (!/^[^\s@]+@[^\s@][^\s.@]*\.[^\s@]+$/.test(formData.email)) {
			errors.email = 'Please enter a valid email';
			isValid = false;
		}

		if (!formData.message.trim()) {
			errors.message = 'Message is required';
			isValid = false;
		}

		return isValid;
	}

	async function handleSubmit() {
		if (!validateForm()) return;

		formStatus = 'submitting';

		try {
			// Replace with your actual form submission logic
			await new Promise((resolve) => setTimeout(resolve, 1500));
			formStatus = 'success';

			// Reset form after success
			setTimeout(() => {
				formData = {
					name: '',
					email: '',
					company: '',
					message: ''
				};
				formStatus = 'idle';
			}, 3000);
		} catch (error) {
			formStatus = 'error';
		}
	}

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

			if (contactSection) {
				observer.observe(contactSection);
			}
		}

		return () => {
			if (contactSection) {
				observer.unobserve(contactSection);
			}
		};
	});
</script>

<section bind:this={contactSection} class="relative overflow-hidden py-24 sm:py-32">
	<!-- Background Elements -->
	<div class="absolute inset-0 -z-10">
		<div class="bg-grid-white/[1] absolute inset-0"></div>
		<div class="absolute left-1/2 top-0 -translate-x-1/2 blur-3xl xl:-top-6">
			<div
				class="aspect-[1155/678] w-[72.1875rem] bg-gradient-to-tr from-primary/30 to-transparent opacity-20"
				style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
			></div>
		</div>
	</div>

	<div class="container relative">
		<div class="mx-auto max-w-2xl lg:max-w-7xl">
			<div class="grid gap-x-8 gap-y-12 lg:grid-cols-2 lg:gap-y-0">
				<div class="lg:pr-8">
					<div
						class="max-w-xl transition-all duration-700 {isInView
							? 'translate-y-0 opacity-100'
							: 'translate-y-4 opacity-0'}"
					>
						<h2 class="text-3xl font-bold tracking-tight sm:text-4xl">Get in touch</h2>
						<p class="mt-6 text-lg leading-8 text-muted-foreground">
							Ready to transform your workflow? We're here to help you get started and answer any
							questions you may have.
						</p>

						<!-- Contact Info -->
						<dl class="mt-10 space-y-4 text-base leading-7">
							<div class="flex gap-x-4">
								<dt class="flex-none">
									<span class="sr-only">Email</span>
									<Icon icon="material-symbols:mail-rounded" class="h-7 w-6 text-primary" />
								</dt>
								<dd>
									<a class="hover:text-primary" href="mailto:support@example.com"
										>support@support.com</a
									>
								</dd>
							</div>
							<div class="flex gap-x-4">
								<dt class="flex-none">
									<span class="sr-only">Location</span>
									<Icon icon="material-symbols:location-on" class="h-7 w-6 text-primary" />
								</dt>
								<dd>Some street<br />Somewhere, DE 58256</dd>
							</div>
						</dl>
					</div>
				</div>

				<div
					class="rounded-2xl border border-border bg-background/50 p-8 backdrop-blur-sm transition-all delay-200 duration-700 {isInView
						? 'translate-y-0 opacity-100'
						: 'translate-y-4 opacity-0'}"
				>
					<form class="space-y-6" onsubmit={handleSubmit}>
						<div>
							<label for="name" class="block text-sm font-medium">Name</label>
							<Input
								type="text"
								id="name"
								bind:value={formData.name}
								class="mt-2 block w-full rounded-md border border-border bg-background/50 px-3.5 py-2 text-foreground shadow-sm ring-1 ring-inset ring-border placeholder:text-muted-foreground focus:ring-2 focus:ring-primary"
								placeholder="Your name"
							/>
							{#if errors.name}
								<p class="mt-1 text-sm text-red-500">{errors.name}</p>
							{/if}
						</div>

						<div>
							<label for="email" class="block text-sm font-medium">Email</label>
							<Input
								type="email"
								id="email"
								bind:value={formData.email}
								class="mt-2 w-full"
								placeholder="you@example.com"
							/>
							{#if errors.email}
								<p class="mt-1 text-sm text-red-500">{errors.email}</p>
							{/if}
						</div>

						<div>
							<label for="company" class="block text-sm font-medium">Company</label>
							<Input
								type="text"
								id="company"
								bind:value={formData.company}
								class="mt-2 w-full"
								placeholder="Your company (optional)"
							/>
						</div>

						<div>
							<label for="message" class="block text-sm font-medium">Message</label>
							<textarea
								id="message"
								bind:value={formData.message}
								rows="4"
								class={cn(inputVariants(), 'h-32 w-full resize-none')}
								placeholder="How can we help you?"
							></textarea>
							{#if errors.message}
								<p class="mt-1 text-sm text-red-500">{errors.message}</p>
							{/if}
						</div>

						<div>
							<button
								type="submit"
								class="flex w-full items-center justify-center rounded-md bg-primary px-3.5 py-2.5 text-center text-sm font-semibold text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary disabled:cursor-not-allowed disabled:opacity-50"
								disabled={formStatus === 'submitting'}
							>
								{#if formStatus === 'submitting'}
									<Icon icon="eos-icons:loading" class="mr-2 h-4 w-4 animate-spin" />
									Sending...
								{:else if formStatus === 'success'}
									<Icon icon="material-symbols:check-circle-rounded" class="mr-2 h-4 w-4" />
									Message sent!
								{:else if formStatus === 'error'}
									<Icon icon="material-symbols:error-rounded" class="mr-2 h-4 w-4" />
									Error sending message
								{:else}
									Send message
								{/if}
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	</div>
</section>
