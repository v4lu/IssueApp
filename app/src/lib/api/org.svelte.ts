import { HTTPError } from 'ky';
import { goto } from '$app/navigation';
import { authAPI } from '$lib/api';
import type { TErrorResponse } from '$lib/types/error.type';
import type { User } from '$lib/types/user.type';
import type { Org as TOrg } from '$lib/types/org.type';

class Org {
	workspace = $state<Org>();
	isCreatingOrg = $state(false);
	isUpdatingOrg = $state(false);
}

export function useOrg(authToken: string) {
	const resp = new Org();
	const api = authAPI(authToken);

	async function createOrg(name: string): Promise<TOrg | undefined> {
		resp.isCreatingOrg = true;
		try {
			const org = await api.post('org', { json: { name } }).json<TOrg>();
			return org;
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		} finally {
			resp.isCreatingOrg = false;
		}
	}

	async function updateOrg(
		orgId: string,
		name: string,
		logo_url: string | null
	): Promise<TOrg | undefined> {
		resp.isUpdatingOrg = true;
		try {
			const org = await api.patch(`org/${orgId}`, { json: { name, logo_url } }).json<TOrg>();
			return org;
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		} finally {
			resp.isUpdatingOrg = false;
		}
	}

	async function deleteOrg(orgId: string): Promise<void> {
		try {
			await api.delete(`org/${orgId}`);
			goto('/orgs');
		} catch (error) {
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;
				console.error(res);
			}
		}
	}

	return {
		resp,
		createOrg,
		updateOrg,
		deleteOrg
	};
}
