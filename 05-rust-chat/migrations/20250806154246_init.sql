-- this file is used for postgresql database initialization
-- create user table
DROP table if exists users;
CREATE TABLE IF NOT EXISTS users
(
    id            bigserial PRIMARY KEY,
    ws_id bigint,
    fullname      varchar(64) NOT NULL,
    email         varchar(64) NOT NULL,
    -- hashed argon2 password, length 97
    password_hash varchar(97) NOT NULL,
    created_at    timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- 插入测试用户数据
INSERT INTO users (ws_id, fullname, email, password_hash) VALUES
(1,'张三', 'zhangsan@example.com', '$argon2id$v=19$m=4096,t=3,p=1$waSRM7HJw7xMIxlG$WDpOvt9hXYRjVyYqgyVx3AcE6lS5DE/Jb6TRfs+BA9w'),
(1,'李四', 'lisi@example.com', '$argon2id$v=19$m=4096,t=3,p=1$waSRM7HJw7xMIxlG$WDpOvt9hXYRjVyYqgyVx3AcE6lS5DE/Jb6TRfs+BA9w'),
(1,'王五', 'wangwu@example.com', '$argon2id$v=19$m=4096,t=3,p=1$waSRM7HJw7xMIxlG$WDpOvt9hXYRjVyYqgyVx3AcE6lS5DE/Jb6TRfs+BA9w');

-- create index for users for email
CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users (email);




-- ----------------------------
-- Records of users
-- ----------------------------


-- create chat type: single, group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM (
    'single',
    'group',
    'private_channel',
    'public_channel'
    );

-- create chat table
CREATE TABLE IF NOT EXISTS chats
(
    id         bigserial PRIMARY KEY,
    name       varchar(128) NOT NULL UNIQUE,
    type       chat_type    NOT NULL,
    -- user id list
    members    bigint[]     NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
);

-- create message table
CREATE TABLE IF NOT EXISTS messages
(
    id         bigserial PRIMARY KEY,
    chat_id    bigint NOT NULL,
    sender_id  bigint NOT NULL,
    content    text   NOT NULL,
    images     text[],
    created_at timestamp DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chats (id),
    FOREIGN KEY (sender_id) REFERENCES users (id)
);

-- create index for messages for chat_id and created_at order by created_at desc
CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages (chat_id, created_at DESC);

-- create index for messages for sender_id
CREATE INDEX IF NOT EXISTS sender_id_index ON messages (sender_id);