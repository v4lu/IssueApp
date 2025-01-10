<script lang="ts">
	import { slide } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import Icon from '@iconify/svelte';
	import { mode, setMode } from 'mode-watcher';
	import { Avatar } from '../ui/avatar';
	import { buttonVariants } from '../ui/button';
	import Button from '../ui/button/button.svelte';
	import { CreateOrg } from '../modals';
	import { session } from '$lib/stores/session.store';
	import { clickOutsideTimeout, cn } from '$lib';

	type TNavigation = {
		name: string;
		href?: string;
		icon: string;
		isDropdown?: boolean;
	};

	type Props = {
		authToken: string;
	};

	let isUserDropdownOpen = $state(false);
	let isOrgDropdownOpen = $state(false);
	let isCreateOrgModalOpen = $state(false);

	let { authToken }: Props = $props();

	let navigations = $state<TNavigation[]>([
		{
			name: 'Settings',
			href: '/settings/preferences',
			icon: 'solar:settings-minimalistic-broken'
		},
		{
			name: 'Workspaces',
			icon: 'solar:box-minimalistic-broken',
			isDropdown: true
		}
	]);

	let workspaces = $state([
		{ name: 'Workspace 1', href: '/workspace/1', icon: 'solar:graph-outline' },
		{ name: 'Workspace 2', href: '/workspace/2', icon: 'solar:graph-outline' }
	]);
</script>

<header class="flex w-full justify-between bg-background-muted p-3">
	<div class="flex w-full items-center justify-start space-x-2">
		<a href="/" class=" text-xl font-bold">Sample</a>
	</div>

	<div class="flex gap-4">
		<Button
			onclick={() => setMode($mode === 'light' ? 'dark' : 'light')}
			variant="outline"
			size="icon"
		>
			<Icon icon={$mode === 'light' ? 'lucide:sun' : 'lucide:moon'} class="size-4" />
		</Button>
		<div class="relative">
			<button
				onclick={() => {
					isUserDropdownOpen = true;
				}}
			>
				<Avatar user={$session} />
			</button>

			{#if isUserDropdownOpen}
				<div
					transition:slide={{ duration: 300, easing: cubicOut }}
					use:clickOutsideTimeout={() => (isUserDropdownOpen = false)}
					class={cn(
						'absolute right-0 top-10 z-30 min-w-[13rem] rounded-md border border-border bg-card shadow-2xl'
					)}
				>
					<div>
						{#each navigations as { name, href, icon, isDropdown }}
							{#if isDropdown}
								<div
									aria-roledescription="menu"
									role="menu"
									tabindex="0"
									class="relative"
									onmouseenter={() => (isOrgDropdownOpen = true)}
									onmouseleave={() => (isOrgDropdownOpen = false)}
								>
									<button
										class={cn(
											buttonVariants({ variant: 'ghost' }),
											'w-full justify-start rounded-none py-5 pl-1 text-sm'
										)}
									>
										{#if isDropdown}
											<Icon icon="material-symbols-light:chevron-left" class="mr-1 size-5" />
										{/if}
										{name}
										<Icon {icon} class="ml-auto size-4" />
									</button>

									{#if isOrgDropdownOpen}
										<div
											class="absolute right-full top-0 min-w-[13rem] rounded-md border border-border bg-card shadow-2xl"
										>
											{#each workspaces as workspace}
												<a
													onclick={() => (isUserDropdownOpen = false)}
													href={workspace.href}
													class={cn(
														buttonVariants({ variant: 'ghost' }),
														'w-full justify-start rounded-none py-5 text-sm'
													)}
												>
													<Icon icon={workspace.icon} class="mr-2 size-4" />
													{workspace.name}
												</a>
											{/each}
											<Button
												onclick={() => {
													isCreateOrgModalOpen = true;
													isOrgDropdownOpen = false;
													isUserDropdownOpen = false;
												}}
												variant="ghost"
												class="w-full justify-start rounded-none border-t border-border py-5 text-sm"
											>
												Create Organization
											</Button>
										</div>
									{/if}
								</div>
							{:else}
								<a
									onclick={() => (isUserDropdownOpen = false)}
									{href}
									class={cn(
										buttonVariants({ variant: 'ghost' }),
										'w-full justify-start rounded-none py-5 pl-[28px] text-sm first:rounded-t-md'
									)}
								>
									{name}
									<Icon {icon} class="ml-auto size-4" />
								</a>
							{/if}
						{/each}

						<div class="border-t border-border">
							<Button
								variant="destructive"
								class="w-full justify-start rounded-b-md bg-transparent py-5 pl-[28px] text-sm text-foreground hover:bg-red-100/40 hover:text-red-400"
							>
								Logout
								<Icon icon="solar:logout-line-duotone" class="ml-auto size-4" />
							</Button>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
</header>

<CreateOrg {authToken} bind:isOpen={isCreateOrgModalOpen} />
