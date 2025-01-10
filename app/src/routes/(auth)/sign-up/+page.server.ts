import { fail, redirect } from '@sveltejs/kit';
import { HTTPError } from 'ky';
import { setError, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { type UserRegisterSchemaPayload, userRegisterSchema } from '$lib/validators/auth.validator';
import { api } from '$lib/api.js';
import type { TErrorResponse } from '$lib/types/error.type';

type RegisterResponse = {
	email: string;
	username: string;
	id: number;
};

export async function load() {
	const form = await superValidate(zod(userRegisterSchema));

	return {
		form
	};
}

export const actions = {
	register: async ({ request }) => {
		const form = await superValidate(request, zod(userRegisterSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		const payload: UserRegisterSchemaPayload = {
			email: form.data.email,
			password: form.data.password,
			username: form.data.username
		};

		let err_happened = false;
		try {
			await api.post<RegisterResponse>('auth/register', {
				json: payload
			});
		} catch (error) {
			err_happened = true;
			if (error instanceof HTTPError) {
				const res = (await error.response.json()) as TErrorResponse;

				console.error(res);
			}
		}

		if (!err_happened) {
			throw redirect(307, '/sign-in');
		}
	}
};
