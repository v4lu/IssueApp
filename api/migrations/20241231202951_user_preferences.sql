-- Add migration script here
CREATE TABLE user_preferences (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    theme VARCHAR(50) NOT NULL DEFAULT 'light',
    language VARCHAR(10) NOT NULL DEFAULT 'en',
    default_org_id UUID REFERENCES org(id) ON DELETE SET NULL,
    cta_color VARCHAR(50) NOT NULL DEFAULT '#34cea7',
    cta_text_color VARCHAR(50) NOT NULL DEFAULT '#310222',
    font_size VARCHAR(50) NOT NULL DEFAULT 'medium',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_preferences_user_id ON user_preferences(user_id);
CREATE INDEX idx_user_preferences_default_org_id ON user_preferences(default_org_id);
