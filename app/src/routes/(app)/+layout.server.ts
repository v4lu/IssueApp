import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { authAPI } from '$lib/api';
import { ACCESS_TOKEN } from '$lib/constants';
import type { User, UserPreference } from '$lib/types/user.type';
import type { Org } from '$lib/types/org.type';

export const load: LayoutServerLoad = async ({ cookies, url }) => {
	const accessToken = cookies.get(ACCESS_TOKEN);

	if (!accessToken) {
		throw redirect(307, '/sign-in');
	}

	const api = authAPI(accessToken);

	const [user, orgs, user_preferences] = await Promise.all([
		api.get('auth').json<User>(),
		api.get('org').json<Org[]>(),
		api.get('user-preferences').json<UserPreference>()
	]);

	const orgExists = orgs.length > 0;

	return {
		user,
		user_preferences,
		accessToken,
		path: url.pathname,
		orgs,
		orgExists
	};
};
