import { error } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { authAPI } from '$lib/api';
import type { MemberRole } from '$lib/types/org.type';

export const load: LayoutServerLoad = async ({ parent, params }) => {
	const { accessToken, orgs } = await parent();
	const org_id = params.id;

	const org = orgs.find((org) => org.id === org_id);

	if (!org) {
		error(404, {
			message: 'Not found'
		});
	}

	const api = authAPI(accessToken);
	const session_role = await api.get(`org/${org_id}/session-role`).json<MemberRole>();

	return {
		org_id,
		org,
		orgs_count: orgs.length,
		session_role
	};
};
