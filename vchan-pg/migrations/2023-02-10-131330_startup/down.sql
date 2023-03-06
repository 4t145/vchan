-- This file should undo anything in `up.sql`
DROP INDEX posts_by_user_id;
DROP INDEX posts_by_thread_id;

DROP TABLE ip_records;
DROP TABLE boards;
DROP TABLE threads;
DROP TABLE posts;
DROP TABLE users;
