const INIT_TEMPLATE = `version: "1"
database:
  migrations:
    - './migrations/*.sql'

queries:
  - "queries/*.sql"

codegen:
  target: python:asyncpg
  out: ./package/queries     # change package to your package name
  options:
    package: package.queries # change package to your package name

`;
export default async function init(filename: string) {
  try {
    const file = await Deno.open(filename, { createNew: true, write: true });
    await file.write(new TextEncoder().encode(INIT_TEMPLATE));
  } catch (error) {
    if ((error as any).code == "EEXIST") {
      console.log(`error: the file ${filename} already exists.`);
    }
  }
}
