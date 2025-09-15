CREATE TABLE IF NOT EXISTS message (
    id INTEGER PRIMARY KEY,
    conversation_id INTEGER NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_by TEXT NOT NULL,

    FOREIGN KEY(conversation_id) REFERENCES conversation(id)
);
