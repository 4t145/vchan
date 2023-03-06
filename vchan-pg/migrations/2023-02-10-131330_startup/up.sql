-- Your SQL goes here


CREATE TABLE users (
    id VARCHAR(16) PRIMARY KEY,
    flag BIGINT NOT NULL,
    create_time TIMESTAMP NOT NULL,
    ban_time TIMESTAMP,
    cookiehash INTEGER[8] NOT NULL
);

CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR(16) NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    flag BIGINT NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    thread_id INTEGER NOT NULL,
    thread_path BIGINT [] NOT NULL,

    email VARCHAR(256),
    title VARCHAR(32),
    author VARCHAR(32)
);

CREATE INDEX posts_by_user_id ON posts(user_id);
CREATE INDEX posts_by_thread_id ON posts(thread_id);

CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    primary_post_id BIGINT NOT NULL REFERENCES posts(id),
    update_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    board_id INTEGER NOT NULL
);

CREATE TABLE boards (
    id SERIAL PRIMARY KEY,
    name VARCHAR(16) NOT NULL UNIQUE
);

CREATE TABLE ip_records (
    ip INET PRIMARY KEY,
    banned BOOLEAN NOT NULL DEFAULT FALSE,
    fetch_cookie_date DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);