CREATE TABLE users_tmp (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_name TEXT UNIQUE NOT NULL,
    user_group_id BIGINT NOT NULL,
    home_path TEXT NOT NULL,
    boot_id TEXT NOT NULL
);

INSERT INTO users_tmp SELECT id, user_name, user_group_id, home_path, boot_id FROM users;
DROP TABLE users;

CREATE TABLE user_group_memberships_tmp (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    group_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL
);

INSERT INTO user_group_memberships_tmp SELECT id, group_id, user_id FROM user_group_memberships;
DROP TABLE user_group_memberships;

CREATE TABLE groups_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    group_name TEXT UNIQUE NOT NULL
);

INSERT INTO groups_new SELECT id, group_name FROM groups;
DROP TABLE groups;
ALTER TABLE groups_new RENAME TO groups;

CREATE TABLE users_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_name TEXT UNIQUE NOT NULL,
    user_group_id BIGINT NOT NULL CONSTRAINT fk_users_groups REFERENCES groups(id),
    home_path TEXT NOT NULL,
    boot_id TEXT NOT NULL
);

INSERT INTO users_new SELECT id, user_name, user_group_id, home_path, boot_id FROM users_tmp;
DROP TABLE users_tmp;
ALTER TABLE users_new RENAME TO users;

CREATE TABLE user_group_memberships_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    group_id BIGINT NOT NULL CONSTRAINT fk_user_group_memberships_group REFERENCES groups(id),
    user_id BIGINT NOT NULL CONSTRAINT fk_user_group_memberships_user REFERENCES users(id)
);

INSERT INTO user_group_memberships_new SELECT id, group_id, user_id FROM user_group_memberships_tmp;
DROP TABLE user_group_memberships_tmp;
ALTER TABLE user_group_memberships_new RENAME TO user_group_memberships;


-- re-create indexes

CREATE INDEX users_user_name_idx ON users(user_name);

CREATE INDEX groups_group_name_idx ON groups(group_name);

CREATE INDEX group_group_memberships_group_id_idx ON user_group_memberships(group_id);
CREATE INDEX group_group_memberships_user_id_idx ON user_group_memberships(user_id);
