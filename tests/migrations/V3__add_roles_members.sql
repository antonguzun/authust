CREATE TABLE IF NOT EXISTS role_members (
    user_id int NOT NULL,
    role_id int NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    is_deleted boolean NOT NULL,

    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(user_id),
    CONSTRAINT fk_roles FOREIGN KEY(role_id) REFERENCES roles(role_id)
);
CREATE UNIQUE INDEX IF NOT EXISTS role_member_unique_binding ON role_members (role_id, user_id);
