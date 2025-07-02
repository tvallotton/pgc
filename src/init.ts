const INIT_TEMPLATE = `
version: "1"

env_file:
  - .env

schema:
  migrations: ./migrations/*.sql
  # pglite:
  #   username: string
  #   database: string
  #   extensions:
  #     vector: @electric-sql/pglite/vector

codegen:
  target: python
  out: ./src/queries
  options:
`;
export default async function init(filename: string) {
  try {
    const file = await Deno.open(filename, { createNew: true, write: true });
    await file.write(new TextEncoder().encode(INIT_TEMPLATE));
  } catch (error) {
    if (error.code == "EEXIST") {
      console.log(`error: the file ${filename} already exists.`);
    }
  }
}
