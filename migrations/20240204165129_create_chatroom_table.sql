CREATE TABLE IF NOT EXISTS chat_rooms
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    name    CHAR(36)                                                        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL
)
