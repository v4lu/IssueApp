import { refreshToken } from '$lib/api';
import { ACCESS_TOKEN, REFRESH_TOKEN, isProduction } from '$lib/constants';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const access = event.cookies.get(ACCESS_TOKEN);
	const refresh = event.cookies.get(REFRESH_TOKEN);

	if (!access && refresh) {
		try {
			const refreshedTokens = await refreshToken(refresh);

			event.cookies.set(ACCESS_TOKEN, refreshedTokens.access_token, {
				path: '/',
				httpOnly: true,
				sameSite: 'strict',
				secure: isProduction,
				maxAge: refreshedTokens.expires_in_access
			});

			event.cookies.set(REFRESH_TOKEN, refreshedTokens.refresh_token, {
				path: '/',
				httpOnly: true,
				sameSite: 'strict',
				secure: isProduction,
				maxAge: refreshedTokens.expires_in_refresh
			});

			event.request.headers.set('Authorization', `Bearer ${refreshedTokens.access_token}`);
		} catch (error) {
			console.error(error);
			event.cookies.delete(ACCESS_TOKEN, { path: '/' });
			event.cookies.delete(REFRESH_TOKEN, { path: '/' });
		}
	}

	if (!refresh) {
		event.cookies.delete(ACCESS_TOKEN, { path: '/' });
		event.cookies.delete(REFRESH_TOKEN, { path: '/' });
	}

	const response = await resolve(event);
	return response;
};
