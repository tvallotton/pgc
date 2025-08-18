# Pgc.yaml
The pgc.yaml file has three main option sections:
* version: configures the output generation version
* queries: configures how the query files will be found in the file system
* database: configures the database that will be used to prepare the queries
* codegen: configures the code generation.

## Version
Currently, the only value supported here is "1".
```
version: 1
```

## Queries
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

    extensions: # a mapping of extensions and their source
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
Pgc also supports connecting to an external development database using a postgres dsn:
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
The codegen section has the following arguments:
* target (required): A predefined target language and driver pair (e.g. "python:asynpg")
* out (required): The output directory
* options (may be required): target specific options (e.g. python requires the `package` option to be defined here.)
* enums (optional): A list of table backed enums
* types (optional): A list of type annotation overrides
* exclude_models: A list of models to exclude from modeling

```yaml
codegen:
  target: python:asyncpg
  out: ./app/queries
  enums:
    - genre
  types:
    pg_catalog.json:
      name: dict
    pg_catalog.geometry:
      name: shapely.Geometry
      annotation: shapely.Geometry
      import: shapely
  options:
    package: app.queries
  exclude_models:
    - logs
```


# Known issues
Asyncpg has some limitations to what fields models can have when setting a type codec. For example, a table containing a `jsonb` field cannot be decoded into a custom class.
This is addressed by excluding the table from the generation.
