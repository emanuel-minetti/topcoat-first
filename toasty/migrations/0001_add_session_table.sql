CREATE TABLE "sessions" (
    "id" UUID NOT NULL,
    "token_hash" BYTEA NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "expires_at" TIMESTAMPTZ(6) NOT NULL,
    "reevoked_at" TIMESTAMPTZ(6),
    PRIMARY KEY ("id")
);
CREATE INDEX "index_sessions_by_user_id" ON "sessions" ("user_id");
