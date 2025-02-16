-- Add migration script here
CREATE TABLE docs (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id uuid NOT NULL REFERENCES org(id) ON DELETE CASCADE,
  creator_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  number integer NOT NULL,
  title text NOT NULL,
  sharable boolean NOT NULL DEFAULT false,
  sharable_link text,
  content jsonb NOT NULL,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now()
);

CREATE INDEX docs_org_id_idx ON docs(org_id);
CREATE INDEX docs_creator_id_idx ON docs(creator_id);
