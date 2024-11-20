## Launch PostgreSql Instance in Docker

```bash
$ cd ~/Projects/Docker/postgresql
$ docker-compose up -d
$ docker-compose ps
```

## Install SQLx-CLI

```bash
# getting help before installing
$ cargo install sqlx-cli --help

# only for postgres, mysql, and sqlite
$ cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite
```

More info about flags: https://lib.rs/crates/sqlx-cli/features

## Perform Database Migrations with SQLx

```bash
$ sqlx migrate add -r users
Creating migrations/20241119235255_users.up.sql
Creating migrations/20241119235255_users.down.sql
```

Open the file `migrations/20241119235255_users.up.sql` and add the following code:

```sql
CREATE TYPE user_role AS ENUM ('admin', 'user');
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    verification_token VARCHAR(255),
    token_expires_at TIMESTAMP WITH TIME ZONE,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX users_email_idx ON users (email);
```

Open the file `migrations/20241119235255_users.down.sql` and add the following code:

```sql
DROP TABLE IF EXISTS "users";
DROP TYPE IF EXISTS user_role;
DROP EXTENSION IF EXISTS "uuid-ossp";
```

> All commands require that a database url is provided. This can be done either with the `--database-url` command line option or by setting `DATABASE_URL`, either in the environment or in a `.env` file in the current working directory.

To apply the 'up' migration script:

```bash
$ sqlx migrate run
```

To apply the 'down' migration script:

```bash
$ sqlx migrate revert
```
