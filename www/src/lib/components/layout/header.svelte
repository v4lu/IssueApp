<script lang="ts">
	import { fly } from 'svelte/transition';
	import Icon from '@iconify/svelte';
	import { buttonVariants } from '../ui';

	let isOpen = $state(false);

	const navItems = $state([
		{
			title: 'Features',
			subitems: ['Task Management', 'Roadmap', 'Issues', 'Analytics'],
			icon: 'material-symbols:space-dashboard'
		},
		{
			title: 'Solutions',
			subitems: ['Engineering', 'Product', 'Design', 'Marketing'],
			icon: 'material-symbols:lightbulb'
		},
		{
			title: 'Resources',
			subitems: ['Documentation', 'Blog', 'Community', 'Support'],
			icon: 'material-symbols:library-books'
		},
		{
			title: 'Pricing',
			icon: 'material-symbols:payments'
		}
	]);
</script>

<header class="fixed top-0 z-50 w-full border-b border-border bg-background/80 backdrop-blur-md">
	<div class="container flex h-16 items-center justify-between">
		<a href="/" class="flex items-center space-x-2">
			<span
				class="bg-gradient-to-r from-primary to-primary/50 bg-clip-text text-xl font-bold text-transparent"
			>
				TSMG
			</span>
		</a>

		<nav class="hidden items-center space-x-8 md:flex">
			{#each navItems as item}
				<div class="group relative">
					<button
						class="flex items-center space-x-2 text-sm text-muted-foreground transition-colors hover:text-foreground"
					>
						<span>{item.title}</span>
						{#if item.subitems}
							<Icon
								icon="material-symbols:keyboard-arrow-down-rounded"
								class="h-4 w-4 transition-transform duration-200 ease-out group-hover:rotate-180"
							/>
						{/if}
					</button>

					{#if item.subitems}
						<div
							class="invisible absolute -left-4 top-full translate-y-1 scale-95 opacity-0 transition-all duration-200 ease-out group-hover:visible group-hover:translate-y-0 group-hover:scale-100 group-hover:opacity-100"
						>
							<div
								class="mt-2 w-56 rounded-xl border border-border bg-popover p-2 shadow-lg backdrop-blur-sm"
							>
								{#each item.subitems as subitem}
									<a
										href="#{subitem.toLowerCase()}"
										class="relative flex items-center rounded-lg px-4 py-2 text-sm text-popover-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
									>
										<span class="relative z-10">{subitem}</span>
										<span
											class="absolute inset-0 rounded-lg bg-primary/0 transition-colors duration-200 hover:bg-primary/5"
										></span>
									</a>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			{/each}
		</nav>

		<div class="hidden items-center space-x-4 md:flex">
			<a href="/sign-in" class={buttonVariants({ variant: 'ghost' })}>
				<Icon icon="material-symbols:login" class="mr-2 h-4 w-4" />
				Sign in
			</a>
			<a href="/sign-up" class={buttonVariants()}>
				<Icon icon="material-symbols:person-add" class="mr-2 h-4 w-4" />
				Sign up
			</a>
		</div>

		<button class="p-2 md:hidden" onclick={() => (isOpen = !isOpen)}>
			<Icon icon={isOpen ? 'material-symbols:close' : 'material-symbols:menu'} class="h-6 w-6" />
		</button>
	</div>

	{#if isOpen}
		<div class="md:hidden" transition:fly={{ y: -20, duration: 200 }}>
			<nav class="container space-y-4 py-4">
				{#each navItems as item}
					<div class="space-y-2">
						<button
							class="flex w-full items-center space-x-2 text-left text-sm font-medium text-foreground"
						>
							<Icon icon={item.icon} class="h-4 w-4" />
							<span>{item.title}</span>
						</button>
						{#if item.subitems}
							<div class="space-y-2 pl-6">
								{#each item.subitems as subitem}
									<a
										href="#{subitem.toLowerCase()}"
										class="flex items-center space-x-2 text-sm text-muted-foreground hover:text-foreground"
									>
										<span class="h-1 w-1 rounded-full bg-muted-foreground"></span>
										<span>{subitem}</span>
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
				<div class="space-y-2 pt-4">
					<a
						href="/sign-in"
						class={buttonVariants({ variant: 'ghost', class: 'w-full justify-center' })}
					>
						<Icon icon="material-symbols:login" class="mr-2 h-4 w-4" />
						Sign in
					</a>
					<a href="/sign-up" class={buttonVariants({ class: 'w-full justify-center' })}>
						<Icon icon="material-symbols:person-add" class="mr-2 h-4 w-4" />
						Sign up
					</a>
				</div>
			</nav>
		</div>
	{/if}
</header>

<style>
	a:hover span.absolute {
		background: linear-gradient(45deg, var(--primary) 0%, transparent 100%);
		opacity: 0.1;
	}
</style>
