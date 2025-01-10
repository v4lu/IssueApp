<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade, fly } from 'svelte/transition';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { Button } from '$lib/components/ui/button';
	import { Field } from '$lib/components/ui/field';
	import { Input } from '$lib/components/ui/input';
	import { toast } from '$lib/stores/toast.store.js';
	import { userLoginSchema } from '$lib/validators/auth.validator.js';
	import { api } from '$lib/api.js';

	let { data } = $props();
	let togglePassword = $state(false);
	const { enhance, form, errors, submitting } = superForm(data.form, {
		validators: zodClient(userLoginSchema),
		onResult: ({ result }) => {
			if (result.status === 500) toast.error('Something went wrong. Please try again');
		}
	});

	async function initGithubLogin() {
		const res = await api.get('auth/oauth/github').json<{ url: string }>();

		if (res) {
			window.location.href = res.url;
		}
	}
</script>

<div class="container mx-auto flex h-full flex-1 items-center justify-center px-4 sm:px-6 lg:px-8">
	<div class="flex w-full justify-center">
		<div class="w-full max-w-xl" in:fly={{ y: 20, duration: 600 }} out:fade>
			<div class="mb-8 text-center">
				<h2
					class="bg-gradient-to-r from-primary via-primary/90 to-primary/80 bg-clip-text text-4xl font-bold text-transparent"
				>
					Welcome Back
				</h2>
				<p class="mt-3 text-muted-foreground">Sign in to your account to continue</p>
			</div>

			<div class="rounded-md border border-border bg-card p-12 shadow-xl backdrop-blur-sm">
				<Button variant="outline" class="w-full" onclick={initGithubLogin}>
					<Icon icon="mdi:github" class="mr-2 h-5 w-5" />
					Continue with GitHub
				</Button>

				<div class="relative my-6">
					<div class="absolute inset-0 flex items-center">
						<div class="w-full border-t border-border/50"></div>
					</div>
					<div class="relative flex justify-center text-sm">
						<span class="bg-card px-4 text-muted-foreground">or continue with email</span>
					</div>
				</div>

				<form use:enhance method="POST" action="?/login" class="space-y-5">
					<Field name="Email" error={$errors.email}>
						<div class="group relative transition-all duration-300">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Icon
									icon="mynaui:envelope"
									class="h-5 w-5 text-muted-foreground/70 transition-colors group-focus-within:text-primary"
								/>
							</div>
							<Input
								bind:value={$form.email}
								type="email"
								name="email"
								id="email"
								placeholder="you@example.com"
								required
								class="border border-border/50 bg-background/50 pl-10 pr-4 ring-primary/20 transition-all duration-300 hover:border-border focus:border-primary/50 focus:bg-background focus:ring-2"
							/>
						</div>
					</Field>

					<Field name="Password" error={$errors.password} class="space-y-2">
						<div class="group relative transition-all duration-300">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Icon
									icon="solar:lock-password-outline"
									class="h-5 w-5 text-muted-foreground/70 transition-colors group-focus-within:text-primary"
								/>
							</div>
							<Input
								bind:value={$form.password}
								type={togglePassword ? 'text' : 'password'}
								name="password"
								placeholder="••••••••"
								id="password"
								required
								class="border border-border/50 bg-background/50 pl-10 pr-10 ring-primary/20 transition-all duration-300 hover:border-border focus:border-primary/50 focus:bg-background focus:ring-2"
							/>
							<button
								type="button"
								class="absolute inset-y-0 right-0 flex items-center pr-3 transition-colors hover:text-primary focus:outline-none"
								onclick={() => (togglePassword = !togglePassword)}
							>
								<Icon
									icon={togglePassword ? 'solar:eye-bold' : 'solar:eye-closed-bold'}
									class="h-5 w-5 text-muted-foreground/70 transition-colors hover:text-primary"
								/>
							</button>
						</div>

						<div class="flex justify-end">
							<a
								href="/forgot-password"
								class="text-sm font-medium text-primary transition-colors hover:text-primary/80 hover:underline"
							>
								Forgot password?
							</a>
						</div>
					</Field>

					<Button
						disabled={$submitting}
						isLoading={$submitting}
						type="submit"
						class="mt-6 w-full bg-primary font-semibold text-primary-foreground shadow-lg transition-all duration-300 hover:bg-primary/90 hover:shadow-primary/25"
						size="lg"
					>
						Sign in
					</Button>
				</form>

				<div class="mt-6 text-center text-sm">
					<span class="text-muted-foreground">Don't have an account?</span>
					<a
						href="/sign-up"
						class="ml-1 font-medium text-primary transition-colors duration-300 hover:text-primary/80 hover:underline"
					>
						Create account
					</a>
				</div>
			</div>
		</div>
	</div>
</div>
