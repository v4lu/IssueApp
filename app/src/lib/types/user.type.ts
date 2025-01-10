export type User = {
	id: string;
	email: string;
	username: string;
	is_email_verified: boolean;
	created_at: string;
	updated_at: string;
	last_login_at: string | null;
	avatar_url: string | null;
	github_id: string | null;
	github_url: string | null;
};

export type UserPreference = {
	id: string;
	user_id: string;
	theme: 'light' | 'dark';
	language: string;
	default_org_id: string | null;
	cta_color: string;
	cta_text_color: string;
	font_size: string;
	created_at: string;
	updated_at: string;
};
