<script lang="ts">
	import { Header, SettingsSidebar } from '$lib/components/layout/index.js';
	import { CreateOrg } from '$lib/components/modals';
	import { orgStore, orgsStore } from '$lib/stores/org.store.js';
	import { session, userPreference } from '$lib/stores/session.store.js';

	let { children, data } = $props();
	session.setUser(data.user);
	userPreference.setUserPreference(data.user_preferences);
	let showNewOrgModal = $state(!data.orgExists);

	if (data.orgExists) {
		orgStore.setOrg(data.orgs[0]);
		orgsStore.setOrgs(data.orgs);
	}
	let showSettingsSidebar = $derived(data.path.startsWith('/settings'));
</script>

<Header authToken={data.accessToken} />
<div class="grid h-[calc(100dvh-75px)] flex-1 bg-background-muted lg:grid-cols-[250px,1fr]">
	{#if showSettingsSidebar}
		<SettingsSidebar path={data.path} />
	{:else}
		<div></div>
	{/if}
	{#if !showNewOrgModal}
		{@render children()}
	{/if}
</div>

<CreateOrg authToken={data.accessToken} isOpen={showNewOrgModal} />
