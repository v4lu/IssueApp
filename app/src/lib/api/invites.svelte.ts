import { HTTPError } from 'ky';
import { authAPI } from '$lib/api';
import type { TErrorResponse } from '$lib/types/error.type';
import type { OrgMemberInviteResponse } from '$lib/types/org.type';

class Invites {
	invites = $state<OrgMemberInviteResponse[]>([]);
	isLoading = $state(true);
	isAnswering = $state(false);
}

export function useInvites(authToken: string) {
	const resp = new Invites();
	const api = authAPI(authToken);

	async function getInvites() {
		try {
			const response = await api.get('invites').json<OrgMemberInviteResponse[]>();
			resp.invites = response;
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isLoading = false;
		}
	}

	async function acceptInvite(token: string, org_id: string) {
		resp.isAnswering = true;
		try {
			await api.post(`invites/${token}/${org_id}/accept`).json();
			resp.invites = resp.invites.filter((invite) => invite.org_member_invite.token !== token);
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isAnswering = false;
		}
	}

	async function cancelInvite(token: string) {
		resp.isAnswering = true;
		try {
			await api.delete(`invites/${token}`).json();
			resp.invites = resp.invites.filter((invite) => invite.org_member_invite.token !== token);
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isAnswering = false;
		}
	}

	getInvites();

	return {
		resp,
		acceptInvite,
		cancelInvite
	};
}
