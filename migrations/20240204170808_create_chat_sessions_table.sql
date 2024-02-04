CREATE TABLE IF NOT EXISTS chat_tokens
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    socket_id    CHAR(36)                                                        NOT NULL,
    token_id      CHAR(36)                                                        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    deleted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NULL
)
