import ky, { type KyInstance } from 'ky';
import { CLIENT_BASE_URL, SERVER_BASE_URL } from './constants';
import type { LoginResponse } from './types/user.type';
import { browser } from '$app/environment';

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

export async function refreshToken(refresh_token: string): Promise<LoginResponse> {
	return await api
		.post<LoginResponse>('auth/refresh', {
			headers: {
				RefreshTokenX: refresh_token
			}
		})
		.json();
}
