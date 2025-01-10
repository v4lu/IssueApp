CREATE TYPE comment_type AS ENUM ('ISSUE', 'DOCUMENT', 'ATTACHMENT');

CREATE TABLE comments (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id uuid NOT NULL REFERENCES org(id) ON DELETE CASCADE,
    creator_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    comment_type comment_type NOT NULL,
    comment_owner_id uuid NOT NULL,  -- id of the issue, document or attachment
    content jsonb NOT NULL,
    parent_id uuid REFERENCES comments(id),
    edited_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX comments_creator_id_idx ON comments(creator_id);
CREATE INDEX comments_parent_id_idx ON comments(parent_id);
CREATE INDEX comments_org_id_idx ON comments(org_id);
CREATE INDEX comments_comment_owner_id_idx ON comments(comment_owner_id);
