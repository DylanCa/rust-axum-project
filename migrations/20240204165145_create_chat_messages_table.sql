CREATE TABLE IF NOT EXISTS chat_messages
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    room_id    CHAR(36)                                                        NOT NULL,
    user_id    CHAR(36)                                                        NOT NULL,
    message      VARCHAR(255)                                                    NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL
)
