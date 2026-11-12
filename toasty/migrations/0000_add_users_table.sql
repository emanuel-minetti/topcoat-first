CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "username" VARCHAR(24) NOT NULL,
    "name" TEXT NOT NULL,
    "language" TEXT NOT NULL,
    "password_hash" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_users_by_username" ON "users" ("username");
-- Insert a default admin user
INSERT INTO "users" ("id", "username", "name", "language", "password_hash")
VALUES ('01a1cd04-6464-71a4-a48a-a9118ff30db3','admin',
        'admin',
        'en', '$argon2id$v=19$m=16,t=2,p=1$Mk5BRUVUVDhqSUZwN25FVA$P8ym5emWHzbeOHXp8Vc41A');