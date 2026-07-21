CREATE TABLE application_settings (
    singleton_id INTEGER PRIMARY KEY CHECK (singleton_id = 1),
    theme TEXT NOT NULL CHECK (theme IN ('system', 'light', 'dark')),
    language TEXT NOT NULL CHECK (length(language) BETWEEN 1 AND 16),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
) STRICT;

INSERT INTO application_settings (
    singleton_id,
    theme,
    language,
    created_at,
    updated_at
)
VALUES (
    1,
    'system',
    'en',
    strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
    strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
);