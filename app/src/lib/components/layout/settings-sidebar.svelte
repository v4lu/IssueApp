<script lang="ts">
	import Icon from '@iconify/svelte';
	import { buttonVariants } from '../ui/button';
	import { cn } from '$lib';
	import { orgsStore } from '$lib/stores/org.store';

	type Props = {
		path: string;
	};

	let { path: currentPath }: Props = $props();

	type Path = {
		name: string;
		href: string;
		icon: string;
	};

	type GroupPath = {
		name: string;

		paths: Path[];
	};
	const paths: GroupPath[] = $state([
		{
			name: 'Account',
			paths: [
				{
					name: 'Preferences',
					href: '/settings/preferences',
					icon: 'typcn:brush'
				},
				{
					name: 'Invites',
					href: '/settings/invites',
					icon: 'solar:mailbox-broken'
				}
			]
		}
	]);
</script>

<aside class="p-4">
	<nav class="space-y-8">
		<div>
			{#each paths as group}
				<h3 class="text-sm font-semibold">{group.name}</h3>
				<ul class="mt-4 space-y-1">
					{#each group.paths as path}
						<li>
							<a
								href={path.href}
								class={cn(
									buttonVariants({
										variant: currentPath === path.href ? 'secondary' : 'ghost'
									}),
									'w-full justify-start'
								)}
							>
								<Icon icon={path.icon} class="mr-2 size-5" />
								<span>{path.name}</span>
							</a>
						</li>
					{/each}
				</ul>
			{/each}
		</div>
		<div>
			<h3 class="text-sm font-semibold">Organizations</h3>
			<ul class="mt-4 space-y-1">
				{#each $orgsStore as org}
					<li>
						<a
							href={`/settings/org/${org.id}`}
							class={cn(
								buttonVariants({
									variant: currentPath.startsWith(`/settings/org/${org.id}`) ? 'secondary' : 'ghost'
								}),
								'w-full justify-start'
							)}
							onclick={() => (currentPath = `/settings/org/${org.id}`)}
						>
							<Icon icon="solar:graph-outline" class="mr-2 size-5" />
							<span class="truncate">{org.name}</span>
						</a>
					</li>
				{/each}
			</ul>
		</div>
	</nav>
</aside>
