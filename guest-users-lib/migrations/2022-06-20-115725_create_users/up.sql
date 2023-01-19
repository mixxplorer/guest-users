CREATE TABLE groups (
    id INTEGER PRIMARY KEY NOT NULL,
    group_name TEXT UNIQUE NOT NULL
) STRICT;

CREATE INDEX groups_group_name_idx ON groups(group_name);


CREATE TABLE user_group_memberships (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    group_id INTEGER NOT NULL CONSTRAINT fk_user_group_memberships_group REFERENCES groups(id),
    user_id INTEGER NOT NULL CONSTRAINT fk_user_group_memberships_user REFERENCES users(id)
) STRICT;

CREATE INDEX group_group_memberships_group_id_idx ON user_group_memberships(group_id);
CREATE INDEX group_group_memberships_user_id_idx ON user_group_memberships(user_id);


CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    user_name TEXT UNIQUE NOT NULL,
    user_group_id INTEGER NOT NULL CONSTRAINT fk_users_groups REFERENCES groups(id),
    home_path TEXT NOT NULL
) STRICT;
CREATE INDEX users_user_name_idx ON users(user_name);
