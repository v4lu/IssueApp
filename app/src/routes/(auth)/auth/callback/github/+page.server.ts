import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { api } from '$lib/api';
import { ACCESS_TOKEN, REFRESH_TOKEN, isProduction } from '$lib/constants';
import type { LoginResponse } from '$lib/types/user.type';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const code = url.searchParams.get('code');

	if (!code) {
		return {
			redirect: '/sign-in'
		};
	}

	const response = await api.get(`auth/oauth/github/login?code=${code}`).json<LoginResponse>();

	const { access_token, refresh_token, access_token_expiration, refresh_token_expiration } =
		response;
	console.log(response);

	cookies.set(ACCESS_TOKEN, access_token, {
		httpOnly: true,
		secure: isProduction,
		sameSite: 'strict',
		path: '/',
		maxAge: access_token_expiration
	});
	cookies.set(REFRESH_TOKEN, refresh_token, {
		httpOnly: true,
		secure: isProduction,
		sameSite: 'strict',
		path: '/',
		maxAge: refresh_token_expiration
	});

	redirect(307, '/');
};
