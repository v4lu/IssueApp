import { writable } from 'svelte/store';
import type { Org } from '$lib/types/org.type';

type OrgStore = {
	subscribe: (this: void, run: (value: Org) => void, invalidate?: () => void) => () => void;
	setOrg: (org: Org) => void;
	updateOrg: (partialOrg: Partial<Org>) => void;
};

type OrgsStore = {
	subscribe: (this: void, run: (value: Org[]) => void, invalidate?: () => void) => () => void;
	addOrg: (org: Org) => void;
	setOrgs: (orgs: Org[]) => void;
	updateOrg: (orgId: string, partialOrg: Partial<Org>) => void;
	deleteOrg: (orgId: string) => void;
};

function createOrgStore(): OrgStore {
	const { subscribe, set, update } = writable<Org>();

	return {
		subscribe,
		setOrg: (org: Org) => set(org),
		updateOrg: (partialOrg: Partial<Org>): void => {
			update((org) => {
				if (org) {
					return { ...org, ...partialOrg };
				}
				return org;
			});
		}
	};
}

function createOrgsStore(): OrgsStore {
	const { subscribe, set, update } = writable<Org[]>([]);

	return {
		subscribe,
		setOrgs: (orgs: Org[]) => set(orgs),
		addOrg: (org: Org): void => {
			update((orgs) => {
				return [...orgs, org];
			});
		},
		updateOrg: (orgId: string, partialOrg: Partial<Org>): void => {
			update((orgs) => {
				return orgs.map((org) => {
					if (org.id === orgId) {
						return { ...org, ...partialOrg };
					}
					return org;
				});
			});
		},
		deleteOrg: (orgId: string): void => {
			update((orgs) => {
				return orgs.filter((org) => org.id !== orgId);
			});
		}
	};
}

export const orgStore = createOrgStore();
export const orgsStore = createOrgsStore();
