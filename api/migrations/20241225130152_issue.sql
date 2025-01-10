-- Add migration script here
CREATE TYPE issue_priority AS ENUM ('URGENT', 'HIGH', 'MEDIUM', 'LOW');
CREATE TYPE issue_status AS ENUM ('BACKLOG', 'TODO', 'IN_PROGRESS', 'IN_REVIEW', 'DONE', 'CANCELED', 'BLOCKED');

CREATE TABLE IF NOT EXISTS issues (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id uuid NOT NULL REFERENCES org(id) ON DELETE CASCADE,
  creator_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  number integer NOT NULL,
  title text NOT NULL,
  description jsonb,
  priority issue_priority DEFAULT 'LOW' NOT NULL,
  status issue_status DEFAULT 'BACKLOG' NOT NULL,
  parent_id uuid REFERENCES issues(id),
  due_date timestamp with time zone,
  created_at timestamp with time zone DEFAULT now(),
  updated_at timestamp with time zone DEFAULT now(),

  CONSTRAINT unique_issue_number_per_org UNIQUE (org_id, number)
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER
LANGUAGE plpgsql
AS
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_issues_updated_at
    BEFORE UPDATE ON issues
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE INDEX issues_creator_id_idx ON issues(creator_id);
CREATE INDEX issues_parent_id_idx ON issues(parent_id);
CREATE INDEX issues_org_id_idx ON issues(org_id);
