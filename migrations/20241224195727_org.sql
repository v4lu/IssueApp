-- Add migration script here
CREATE TYPE member_role AS ENUM ('OWNER', 'ADMIN', 'MEMBER');
CREATE TYPE member_status AS ENUM ('ACTIVE', 'INVITED', 'DISABLED');

CREATE TABLE IF NOT EXISTS org (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    slug text NOT NULL UNIQUE,
    custom_id text NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    logo_url text,

    CONSTRAINT valid_slug CHECK (slug ~* '^[a-z0-9-]+$')
);

CREATE TABLE IF NOT EXISTS org_members (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id uuid NOT NULL REFERENCES org(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role member_role NOT NULL DEFAULT 'MEMBER',
    status member_status NOT NULL DEFAULT 'INVITED',
    joined_at timestamp with time zone DEFAULT now(),
    invited_by uuid NOT NULL REFERENCES users(id),
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS org_invites (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id uuid NOT NULL REFERENCES org(id) ON DELETE CASCADE,
    email text NOT NULL UNIQUE,
    role member_role NOT NULL DEFAULT 'MEMBER',
    invited_by uuid NOT NULL REFERENCES users(id),
    invited_at timestamp with time zone DEFAULT now(),
    expires_at timestamp with time zone NOT NULL,
    token text NOT NULL UNIQUE,

    CONSTRAINT valid_invite_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

ALTER TABLE users ADD CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$');

CREATE INDEX IF NOT EXISTS org_slug_idx ON org(slug);

CREATE INDEX IF NOT EXISTS org_members_user_id_idx ON org_members(user_id);
CREATE INDEX IF NOT EXISTS org_members_org_id_idx ON org_members(org_id);
CREATE INDEX IF NOT EXISTS org_members_role_idx ON org_members(role);
CREATE INDEX IF NOT EXISTS org_members_status_idx ON org_members(status);

CREATE INDEX IF NOT EXISTS org_invites_token_idx ON org_invites(token);
CREATE INDEX IF NOT EXISTS org_invites_email_idx ON org_invites(email);
CREATE INDEX IF NOT EXISTS org_invites_org_id_idx ON org_invites(org_id);
