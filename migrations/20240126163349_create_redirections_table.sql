-- Add migration script here
CREATE TABLE IF NOT EXISTS redirections
(
    id         CHAR(36) PRIMARY KEY                                            NOT NULL,
    user_id    CHAR(36)                                                        NOT NULL,
    shortcode      VARCHAR(255)                                                    NOT NULL,
    url    VARCHAR(1024)                                                      NOT NULL,
    utilizations    INTEGER                                                      NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL
)-- Add migration script here
