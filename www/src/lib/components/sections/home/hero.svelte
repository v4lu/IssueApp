<script lang="ts">
	import Icon from '@iconify/svelte';
	import { buttonVariants } from '$lib/components/ui';
</script>

<section class="relative overflow-hidden">
	<div class="container relative pt-24 md:pt-48">
		<div class="mx-auto max-w-4xl text-center">
			<div class="animate-in" style="--stagger: 1">
				<span
					class="inline-flex items-center rounded-full border border-primary/20 bg-background/80 px-3 py-1 text-sm leading-6 text-primary backdrop-blur-sm transition-colors hover:border-primary/40 hover:bg-background"
				>
					<span class="flex h-2 w-2">
						<span
							class="absolute inline-flex h-2 w-2 animate-ping rounded-full bg-primary opacity-75"
						></span>
						<span class="relative inline-flex h-2 w-2 rounded-full bg-primary"></span>
					</span>
					<span class="ml-3 font-medium">Private Beta Coming Soon</span>
					<Icon icon="material-symbols:arrow-forward-rounded" class="ml-1 h-4 w-4" />
				</span>
			</div>

			<h1
				class="mt-8 animate-in text-4xl font-bold tracking-tight sm:text-6xl md:text-7xl"
				style="--stagger: 2"
			>
				<span
					class="bg-gradient-to-r from-foreground via-foreground/90 to-foreground/70 bg-clip-text text-transparent"
				>
					The Future of
				</span>
				<br />
				<span
					class="relative bg-gradient-to-r from-primary via-primary/90 to-primary/70 bg-clip-text text-transparent"
				>
					Task Management
					<span class="absolute -right-8 top-0 h-3 w-3">
						<span
							class="absolute inline-flex h-full w-full animate-ping rounded-full bg-primary opacity-75"
						></span>
						<span class="relative inline-flex h-full w-full rounded-full bg-primary"></span>
					</span>
				</span>
			</h1>

			<p
				class="mt-6 animate-in text-balance text-lg leading-8 text-muted-foreground md:text-lg"
				style="--stagger: 3"
			>
				Join the waitlist for early access to the most intuitive project management platform. Built
				for modern teams who demand excellence.
			</p>

			<div
				class="mt-10 flex animate-in flex-col items-center justify-center gap-4 sm:flex-row"
				style="--stagger: 4"
			>
				<a href="/waitlist" class={buttonVariants({ size: 'lg' })}>
					<Icon icon="material-symbols:rocket-launch" class="mr-2 h-5 w-5" />
					Join Waitlist
				</a>
				<a href="/preview" class={`${buttonVariants({ variant: 'outline', size: 'lg' })} group`}>
					<Icon
						icon="material-symbols:play-circle"
						class="mr-2 h-5 w-5 transition-transform group-hover:scale-110"
					/>
					Preview Demo
				</a>
			</div>
		</div>

		<div class="perspective-1000 mt-16 animate-in sm:mt-24" style="--stagger: 6">
			<div
				class="hover:rotate-x-1 relative mx-auto max-w-6xl rounded-xl border border-border bg-background/50 shadow-2xl backdrop-blur-sm transition-transform hover:scale-[1.01]"
			>
				<div class="flex items-center justify-between border-b border-border p-4">
					<div class="flex items-center space-x-2">
						<div class="h-3 w-3 rounded-full bg-destructive"></div>
						<div class="h-3 w-3 rounded-full bg-primary"></div>
						<div class="h-3 w-3 rounded-full bg-secondary"></div>
					</div>
				</div>

				<div class="grid grid-cols-12 gap-4 p-8">
					<div class="col-span-3 space-y-4">
						{#each Array(5) as _, i}
							<div
								class="h-8 animate-pulse rounded-lg bg-accent/50 transition-all hover:bg-accent/70"
								style="animation-delay: {i * 0.2}s"
							></div>
						{/each}
					</div>

					<div class="col-span-9 space-y-6">
						<div
							class="h-12 rounded-lg bg-accent/30 transition-all hover:bg-accent/50"
							style="animation: pulse 4s ease-in-out infinite"
						></div>
						<div class="grid gap-4">
							{#each Array(3) as _, i}
								<div
									class="h-24 rounded-lg bg-accent/20 transition-all hover:bg-accent/40"
									style="animation: pulse 4s ease-in-out infinite; animation-delay: {i * 0.3}s"
								></div>
							{/each}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.animate-in {
		animation: in 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
		opacity: 0;
		transform: translateY(10px);
		animation-delay: calc(var(--stagger) * 100ms);
	}

	@keyframes in {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
