import type { RequestEvent } from "./$types";

export function GET({ url }: RequestEvent) {
  let s: number[] = Array.from({ length: 5000000 }, () =>
    Math.floor(Math.random() * 40)
  );
  const t0 = performance.now();
  s.sort();
  const t1 = performance.now();
  return new Response(`${t1 - t0} ms`);
}
