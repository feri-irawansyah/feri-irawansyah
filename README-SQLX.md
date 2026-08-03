# SQLx CLI Documentation

Panduan penggunaan SQLx CLI untuk project Rust dengan PostgreSQL.

## Installation

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Cek versi:

```bash
sqlx --version
```

## Environment Variable

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/mydb
```

### Git Bash

```bash
export DATABASE_URL=postgres://postgres:password@localhost:5432/mydb
```

### PowerShell

```powershell
$env:DATABASE_URL="postgres://postgres:password@localhost:5432/mydb"
```

## Create Database

```bash
sqlx database create
```

## Drop Database

```bash
sqlx database drop -y
```

## Create Migration

```bash
sqlx migrate add create_users
```

## Create Reversible Migration

```bash
sqlx migrate add -r create_users
```

Hasil:

```text
migrations/
├── timestamp_create_users.up.sql
└── timestamp_create_users.down.sql
```

## Run Migration

```bash
sqlx migrate run
```

## Migration Status

```bash
sqlx migrate info
```

## Revert Last Migration

```bash
sqlx migrate revert
```

## Revert Jump Vertion

```bash
sqlx migrate revert --target-version 1
```

## Reset All Migrations

```bash
sqlx migrate reset
```

## Custom Migration Folder

```bash
sqlx migrate run --source ./src/nexus_migration/migrations
sqlx migrate revert --source ./src/nexus_migration/migrations
sqlx migrate info --source ./src/nexus_migration/migrations
```

## Offline Mode

```bash
cargo sqlx prepare --workspace
```

## Verify Queries

```bash
cargo sqlx prepare --check
```

## Recommended Workflow

1. Create migration

```bash
sqlx migrate add -r create_users
```

2. Isi `.up.sql`
3. Isi `.down.sql`

4. Run migration

```bash
sqlx migrate run
```

5. Cek status

```bash
sqlx migrate info
```

6. Jika salah

```bash
sqlx migrate revert
```

7. Perbaiki migration
8. Jalankan lagi

```bash
sqlx migrate run
```
