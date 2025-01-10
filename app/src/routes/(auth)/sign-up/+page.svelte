<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade, fly } from 'svelte/transition';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { Button } from '$lib/components/ui/button';
	import { Field } from '$lib/components/ui/field';
	import { Input } from '$lib/components/ui/input';
	import { userRegisterSchema } from '$lib/validators/auth.validator.js';

	let { data } = $props();
	let togglePassword = $state(false);
	let toggleConfirmPassword = $state(false);
	const { enhance, form, errors, submitting } = superForm(data.form, {
		validators: zodClient(userRegisterSchema)
	});

	function getPasswordStrength(password: string) {
		if (!password) return 0;
		let score = 0;
		if (password.length >= 8) score += 20;
		if (/[a-z]/.test(password)) score += 20;
		if (/[A-Z]/.test(password)) score += 20;
		if (/\d/.test(password)) score += 20;
		if (/[@$!%*?&.]/.test(password)) score += 20;
		return score;
	}

	let passwordStrength = $derived(getPasswordStrength($form.password));
</script>

<div class="grid min-h-screen w-full">
	<div class="flex w-full items-center justify-center p-4 lg:p-8">
		<div class="w-full max-w-2xl" in:fly={{ y: 20, duration: 600 }} out:fade>
			<div class="mb-8 space-y-6 text-center lg:hidden">
				<div class="flex flex-col items-center justify-center">
					<h2
						class="bg-gradient-to-r from-primary via-primary/90 to-primary/80 bg-clip-text text-3xl font-bold text-transparent"
					>
						Join Us
					</h2>
					<p class="mt-2 text-muted-foreground">Begin your journey with us today</p>
				</div>
			</div>

			<div
				class="w-full space-y-6 rounded-2xl border border-border/50 bg-card/95 p-8 shadow-2xl ring-1 ring-border/50 backdrop-blur-sm transition-all duration-300 hover:shadow-lg lg:p-12"
			>
				<form use:enhance method="POST" action="?/register" class="w-full space-y-6">
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

					<Field name="Username" error={$errors.username}>
						<div class="group relative transition-all duration-300">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Icon
									icon="mdi:user"
									class="h-5 w-5 text-muted-foreground/70 transition-colors group-focus-within:text-primary"
								/>
							</div>
							<Input
								bind:value={$form.username}
								type="text"
								name="username"
								id="username"
								placeholder="johndoe"
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

						{#if $form.password}
							<div class="h-1.5 w-full overflow-hidden rounded-full bg-muted/50" transition:fade>
								<div
									class="h-full transition-all duration-500 ease-out {passwordStrength <= 40
										? 'bg-red-500'
										: passwordStrength <= 80
											? 'bg-yellow-500'
											: 'bg-green-500'}"
									style="width: {passwordStrength}%"
								></div>
							</div>
						{/if}
					</Field>

					<Field name="Confirm Password" error={$errors.confirmPassword}>
						<div class="group relative transition-all duration-300">
							<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
								<Icon
									icon="solar:lock-password-outline"
									class="h-5 w-5 text-muted-foreground/70 transition-colors group-focus-within:text-primary"
								/>
							</div>
							<Input
								bind:value={$form.confirmPassword}
								type={toggleConfirmPassword ? 'text' : 'password'}
								name="confirmPassword"
								id="confirmPassword"
								placeholder="••••••••"
								required
								class="border border-border/50 bg-background/50 pl-10 pr-10 ring-primary/20 transition-all duration-300 hover:border-border focus:border-primary/50 focus:bg-background focus:ring-2"
							/>
							<button
								type="button"
								class="absolute inset-y-0 right-0 flex items-center pr-3 transition-colors hover:text-primary focus:outline-none"
								onclick={() => (toggleConfirmPassword = !toggleConfirmPassword)}
							>
								<Icon
									icon={toggleConfirmPassword ? 'solar:eye-bold' : 'solar:eye-closed-bold'}
									class="h-5 w-5 text-muted-foreground/70 transition-colors hover:text-primary"
								/>
							</button>
						</div>
					</Field>

					<Button
						disabled={$submitting}
						isLoading={$submitting}
						type="submit"
						class="w-full bg-primary font-semibold text-primary-foreground shadow-lg transition-all duration-300 hover:bg-primary/90 hover:shadow-primary/25"
						size="lg"
					>
						Create account
					</Button>
				</form>

				<div class="mt-6">
					<div class="relative">
						<div class="absolute inset-0 flex items-center">
							<div class="w-full border-t border-border/50"></div>
						</div>
						<div class="relative flex justify-center text-sm">
							<span class="bg-card px-4 text-muted-foreground">Already have an account?</span>
						</div>
					</div>

					<div class="mt-6 flex justify-center">
						<a
							href="/sign-in"
							class="font-medium text-primary transition-colors duration-300 hover:text-primary/80 hover:underline"
						>
							Sign in
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
