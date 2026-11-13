CREATE TYPE "language" AS ENUM ('de', 'en');
CREATE TABLE "users"
(
    "id"            UUID        NOT NULL,
    "username"      VARCHAR(24) NOT NULL,
    "name"          TEXT        NOT NULL,
    "language"      "language"  NOT NULL,
    "password_hash" TEXT        NOT NULL,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "index_users_by_username" ON "users" ("username");

INSERT INTO "users" ("id", "username", "name", "language", "password_hash")
    VALUES ('01a1d6a7-e9a7-7479-b5cb-fda6a3e89ceb', 'admin', 'admin', 'de',
        '$argon2id$v=19$m=16,t=2,p=1$dWtJRlhoamJCNnFmVkFRTg$ThD0/riSeZOTfzojicWB8Q');
