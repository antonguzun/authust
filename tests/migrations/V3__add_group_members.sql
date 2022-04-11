CREATE TABLE IF NOT EXISTS group_members (
    user_id int NOT NULL,
    group_id int NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    is_deleted boolean NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(user_id),
    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES groups(group_id)
);
CREATE UNIQUE INDEX IF NOT EXISTS group_member_unique_binding ON group_members (group_id, user_id);
