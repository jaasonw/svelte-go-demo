import type { RequestEvent } from "../api/$types";

export function GET({ url }: RequestEvent) {
  let s: string[] = [];
  const t0 = performance.now();
  for (let i = 0; i < 5000; i++) {
    // s = [...s, "test"];
    s.push("test");
  }
  const t1 = performance.now();
  return new Response(`${t1 - t0} ms`);
}
