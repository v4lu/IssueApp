<script lang="ts">
	import { setMode } from 'mode-watcher';
	import { CardWrapper, DefaultWrapper, SettingsWrapper } from '$lib/components/layout';
	import { Dropdown } from '$lib/components/ui/dropdown';
	import { userPreference } from '$lib/stores/session.store';
	import { capitalize } from '$lib';
	import { Button } from '$lib/components/ui/button';
	import { SettingsCard } from '$lib/components/cards';
	import { authAPI } from '$lib/api';

	let { data } = $props();
	let isOpenThemeDropdown = $state(false);
	let isOpenLanguageDropdown = $state(false);
	let isOpenDefaultOrgDropdown = $state(false);
	let isUpdating = $state(false);

	function getOrg() {
		if (!$userPreference.default_org_id) return;
		let org = data.orgs.find((org) => org.id === $userPreference.default_org_id);
		return org;
	}

	async function update() {
		isUpdating = true;
		const api = authAPI(data.accessToken);

		const payload = {
			theme: $userPreference.theme,
			language: $userPreference.language,
			default_org_id: $userPreference.default_org_id,
			cta_color: $userPreference.cta_color,
			cta_text_color: $userPreference.cta_text_color,
			font_size: $userPreference.font_size
		};
		await api.patch('user-preferences', {
			json: payload
		});
		isUpdating = false;
	}
</script>

<DefaultWrapper>
	<SettingsWrapper title="Preferences">
		<CardWrapper class="grid gap-8">
			<SettingsCard
				title="Theme"
				description="Change the appearance of the app."
				icon="solar:moon-stars-linear"
			>
				<Dropdown
					triggerText={capitalize($userPreference.theme)}
					triggerClass="min-w-0 w-fit"
					downArrowIcon
					class="right-0 top-[calc(100%+4px)] min-w-32"
					bind:isOpen={isOpenThemeDropdown}
				>
					<Button
						size="sm"
						onclick={() => {
							userPreference.updateUserPreference({ theme: 'light' });
							setMode('light');
							isOpenThemeDropdown = false;
						}}
						class="w-full justify-start"
						variant="ghost">Light</Button
					>
					<Button
						size="sm"
						onclick={() => {
							userPreference.updateUserPreference({ theme: 'dark' });
							setMode('dark');
							isOpenThemeDropdown = false;
						}}
						class="w-full justify-start"
						variant="ghost">Dark</Button
					>
				</Dropdown>
			</SettingsCard>
			<SettingsCard
				title="Language"
				description="Change the language of the app."
				icon="solar:planet-2-line-duotone"
			>
				<Dropdown
					triggerText={capitalize($userPreference.language)}
					triggerClass="min-w-0 w-fit"
					downArrowIcon
					class="right-0 top-[calc(100%+4px)] min-w-32"
					bind:isOpen={isOpenLanguageDropdown}
				>
					<Button
						size="sm"
						onclick={() => {
							userPreference.updateUserPreference({ language: 'en' });
							isOpenLanguageDropdown = false;
						}}
						class="w-full justify-start"
						variant="ghost">English</Button
					>
				</Dropdown>
			</SettingsCard>
			<SettingsCard
				title="Default Organization"
				description="Change the default organization."
				icon="solar:bolt-linear"
			>
				<Dropdown
					triggerText={getOrg()?.name! || 'Select Organization'}
					triggerClass="min-w-0 w-fit"
					downArrowIcon
					class="right-0 top-[calc(100%+4px)] min-w-32"
					bind:isOpen={isOpenDefaultOrgDropdown}
				>
					{#each data.orgs as org}
						<Button
							size="sm"
							onclick={() => {
								userPreference.updateUserPreference({ default_org_id: org.id });
								isOpenDefaultOrgDropdown = false;
							}}
							class="w-full justify-start"
							variant="ghost">{org.name}</Button
						>
					{/each}
				</Dropdown>
			</SettingsCard>

			<div class="flex w-full justify-end">
				<Button disabled={isUpdating} onclick={update}>Save</Button>
			</div>
		</CardWrapper>
	</SettingsWrapper>
</DefaultWrapper>
