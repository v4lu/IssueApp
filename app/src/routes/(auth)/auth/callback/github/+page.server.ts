import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { api } from '$lib/api';
import { ACCESS_TOKEN, REFRESH_TOKEN, isProduction } from '$lib/constants';

type LoginResponse = {
	access_token: string;
	refresh_token: string;

	expires_in_refresh: number;
	expires_in_access: number;
};

export const load: PageServerLoad = async ({ url, cookies }) => {
	const code = url.searchParams.get('code');

	if (!code) {
		return {
			redirect: '/sign-in'
		};
	}

	const response = await api.get(`auth/oauth/github/login?code=${code}`).json<LoginResponse>();

	const { access_token, refresh_token, expires_in_refresh, expires_in_access } = response;

	cookies.set(ACCESS_TOKEN, access_token, {
		httpOnly: true,
		secure: isProduction,
		sameSite: 'strict',
		path: '/',
		maxAge: expires_in_access
	});
	cookies.set(REFRESH_TOKEN, refresh_token, {
		httpOnly: true,
		secure: isProduction,
		sameSite: 'strict',
		path: '/',
		maxAge: expires_in_refresh
	});

	redirect(307, '/');
};
