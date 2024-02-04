CREATE TABLE IF NOT EXISTS chat_tokens
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    user_id    CHAR(36)                                                        NOT NULL,
    token      CHAR(36)                                                        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL
)
