import type { RequestEvent } from "./$types";

export async function GET({ url }: RequestEvent) {
  const req = await fetch(
    "https://pokeapi.co/api/v2/pokemon?limit=100000&offset=0"
  );
  const jsonStr = await req.text();

  const t0 = performance.now();
  JSON.parse(jsonStr);
  const t1 = performance.now();

  return new Response(`${t1 - t0} ms`);
}
