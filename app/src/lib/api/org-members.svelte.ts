import { HTTPError } from 'ky';
import { authAPI } from '$lib/api';
import type { TErrorResponse } from '$lib/types/error.type';
import type { User } from '$lib/types/user.type';
import type { MemberRole, OrgMemberInvite, OrgMemberWithUser } from '$lib/types/org.type';

class OrgMembers {
	members = $state<OrgMemberWithUser[]>([]);
	memberInvites = $state<OrgMemberInvite[]>([]);
	isLoading = $state(true);
	isInviting = $state(false);
	isRemoving = $state(false);
	isUpdatingRole = $state(false);
}

export function useOrgMembers(authToken: string, orgId: string) {
	const resp = new OrgMembers();
	const api = authAPI(authToken);

	async function getMembers() {
		try {
			const response = await api.get(`org/${orgId}/members`).json<OrgMemberWithUser[]>();
			resp.members = response;
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		}
	}

	async function getInvites() {
		try {
			const response = await api.get(`org/${orgId}/invites`).json<OrgMemberInvite[]>();
			resp.memberInvites = response;
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		}
	}

	async function loadData() {
		resp.isLoading = true;
		await Promise.all([getMembers(), getInvites()]);
		resp.isLoading = false;
	}

	async function addMember(email: string) {
		resp.isInviting = true;
		try {
			await api.post(`org/${orgId}/invites`, { json: { email } }).json();
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isInviting = false;
		}
	}

	async function removeMember(member_id: string) {
		resp.isRemoving = true;
		try {
			await api.delete(`org/${orgId}/member/${member_id}`).json();
			resp.members = resp.members.filter((m) => m.org_member.user_id !== member_id);
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isRemoving = false;
		}
	}

	async function updateRole(member_id: string, role: MemberRole) {
		resp.isUpdatingRole = true;
		try {
			await api.patch(`org/${orgId}/member/${member_id}`, { json: { role } }).json();

			const member = resp.members.find((m) => m.org_member.user_id === member_id);
			if (member) {
				member.org_member.role = role;
			}
			resp.members = resp.members.map((m) => {
				if (m.org_member.user_id === member_id) {
					return {
						...m,
						org_member: {
							...m.org_member,
							role
						}
					};
				}
				return m;
			});
		} catch (error) {
			if (error instanceof HTTPError) {
				const e = (await error.response.json()) as TErrorResponse;
				console.error(e);
			}
		} finally {
			resp.isUpdatingRole = false;
		}
	}

	loadData();

	return {
		resp,
		addMember,
		removeMember,
		updateRole
	};
}
