CREATE TABLE rooms (
    "id"            BIGSERIAL   PRIMARY KEY,
    "name"          VARCHAR NOT NULL UNIQUE,
    "description"   VARCHAR,
    "is_public"     BOOLEAN NOT NULL DEFAULT 't'
);

CREATE UNIQUE INDEX unique_name_on_rooms ON rooms (lower(name));

CREATE TABLE videos (
    "id"            BIGSERIAL   PRIMARY KEY,
    "video_id"      VARCHAR     NOT NULL,
    "title"         VARCHAR     NOT NULL,
    "description"   VARCHAR,
    "room_id"       BIGSERIAL   REFERENCES rooms (id) ON DELETE CASCADE,
    "duration"      VARCHAR     NOT NULL,
    "played"        BOOLEAN     NOT NULL DEFAULT 'f',
    "added_on"      TIMESTAMP   NOT NULL DEFAULT now(),
    "started_on"    TIMESTAMP   DEFAULT NULL
);

CREATE TABLE users (
    "id"            BIGSERIAL   PRIMARY KEY,
    "username"      VARCHAR     NOT NULL UNIQUE,
    "password"      VARCHAR     NOT NULL,
    "added_on"      TIMESTAMP   NOT NULL DEFAULT now(),
    "updated_at"    TIMESTAMP   DEFAULT NULL
);

CREATE UNIQUE INDEX unique_username_on_users ON users (lower(username));