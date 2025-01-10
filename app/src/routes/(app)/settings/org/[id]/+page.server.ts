import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, parent }) => {
	const { orgs } = await parent();
	const org_id = params.id;

	const org = orgs.find((org) => org.id === org_id);

	if (!org) {
		error(404, {
			message: 'Not found'
		});
	}

	return {
		org_id,
		org,
		orgs_count: orgs.length
	};
};
