import type { User } from './user.type';

export type MemberRole = 'OWNER' | 'ADMIN' | 'MEMBER';
export type MemberStatus = 'Active' | 'Invited' | 'Disabled';

export type Org = {
	id: string;
	name: string;
	slug: string;
	custom_id: string;
	created_at: string;
	updated_at: string;
	logo_url: string | null;
};

export type OrgMember = {
	id: string;
	org_id: string;
	user_id: string;
	role: MemberRole;
	status: MemberStatus;
	joined_at: string;
	invited_by: string;
	created_at: string;
	updated_at: string;
};

export type OrgMemberWithUser = {
	user: User;
	org_member: OrgMember;
};

export type OrgMemberInvite = {
	id: string;
	org_id: string;
	email: string;
	role: MemberRole;
	invited_by: string;
	invited_at: string;
	expires_at: string;
	token: string;
};

export type OrgMemberInviteResponse = {
	org_member_invite: OrgMemberInvite;
	org_name: string;
	org_logo: string | null;
	invited_by: User;
};
