import { browser } from '$app/environment';
import ky, { type KyInstance } from 'ky';
import { CLIENT_BASE_URL, SERVER_BASE_URL } from './constants';

function getBaseUrl(): string {
	return browser ? CLIENT_BASE_URL : SERVER_BASE_URL;
}

export const api = ky.create({
	prefixUrl: getBaseUrl()
});

export function authAPI(authToken: string): KyInstance {
	return ky.create({
		prefixUrl: getBaseUrl(),
		headers: {
			Authorization: `Bearer ${authToken}`
		},
		retry: {
			limit: 2,
			methods: ['get', 'post', 'put', 'delete', 'patch'],
			statusCodes: [500]
		}
	});
}

type RefreshResponse = {
	access_token: string;
	refresh_token: string;
	expires_in_refresh: number;
	expires_in_access: number;
};

export async function refreshToken(refresh_token: string): Promise<RefreshResponse> {
	return await api
		.post<RefreshResponse>('auth/refresh', {
			headers: {
				RefreshTokenX: refresh_token
			}
		})
		.json();
}
