-- Add migration script here
CREATE TABLE IF NOT EXISTS notes
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    user_id    CHAR(36)                                                        NOT NULL,
    title      VARCHAR(255)                                                    NOT NULL,
    content    TEXT(1024)                                                      NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL
)-- Add migration script here
