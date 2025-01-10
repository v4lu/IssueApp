import { api } from '$lib/api.js';
import { ACCESS_TOKEN, REFRESH_TOKEN, isProduction } from '$lib/constants.js';
import { type UserLoginSchemaPayload, userLoginSchema } from '$lib/validators/auth.validator.js';
import { fail, redirect } from '@sveltejs/kit';
import { HTTPError } from 'ky';
import { setError, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import type { Action } from './$types';

type LoginResponse = {
	access_token: string;
	refresh_token: string;

	expires_in_refresh: number;
	expires_in_access: number;
};

export async function load() {
	const form = await superValidate(zod(userLoginSchema));

	return {
		form
	};
}

export const actions = {
	login: async ({ request, cookies }) => {
		const form = await superValidate(request, zod(userLoginSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const payload: UserLoginSchemaPayload = {
			email: form.data.email,
			password: form.data.password
		};

		try {
			const res = await api
				.post<LoginResponse>('auth/login', {
					json: payload
				})
				.json();

			const { access_token, refresh_token, expires_in_refresh, expires_in_access } = res;

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
		} catch (error) {
			if (error instanceof HTTPError) {
				if (error.response.status === 403) {
					return setError(form, 'email', 'Invalid email or password');
				}
				if (error.response.status === 401) {
					return setError(form, 'email', 'Invalid email or password');
				} else {
					return fail(500, { form });
				}
			}
		}

		redirect(307, '/');
	}
};
