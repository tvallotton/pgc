## Pgc reference

### Pgc.yaml
The pgc.yaml file has three main option sections:
* queries: configures how the query files will be found in the file system
* database: configures the database that will be used to prepare the queries
* codegen: configures the code generation.

#### Queries
Queries can either be a unix glob pattern or an array of patterns.
```yaml
queries:
  - 'queries/*.sql'
```

### In-Memory Database
Pgc uses glite to run an in-memory postgres database. The pglite configuration accepts the following options:
```yaml
database:
  pglite:
    username: postgres
    database: postgres
    extensions:
      pg_trgm: "@electric-sql/pglite/contrib/pg_trgm"
      # for more extensions see https://pglite.dev/extensions/
```

When using an in-memory database, at least one database migration glob pattern must be defined.
```yaml
database:
  migrations:
    - 'migrations/*.up.sql'
```
Migrations will be run in alphabetical order.

### External Database
Pgc also supports connecting to an external development database using a postgres uri:
```yaml
database:
  url: postgres://postgres:password@127.0.0.1:5432/database
```
If the database url cannot be committed to version control, use an environment variable:
```yaml
database:
  url: $DATABASE_URL
```

## Codegen
