import type { RequestEvent } from "@sveltejs/kit";

export function GET({ url }: RequestEvent) {
  const t0 = performance.now();
  // https://www.geeksforgeeks.org/estimating-value-pi-using-monte-carlo/
  let INTERVAL = 10000;
  let rand_x, rand_y, origin_dist, pi;
  let circle_points = 0,
    square_points = 0;

  for (let i = 0; i < INTERVAL * INTERVAL; i++) {
    rand_x = (Math.random() * INTERVAL) / INTERVAL;
    rand_y = (Math.random() * INTERVAL) / INTERVAL;
    origin_dist = rand_x * rand_x + rand_y * rand_y;
    if (origin_dist <= 1) circle_points++;
    square_points++;
    pi = (4 * circle_points) / square_points;
  }
  const t1 = performance.now();
  console.log(pi);
  return new Response(`${t1 - t0} ms`);
}
