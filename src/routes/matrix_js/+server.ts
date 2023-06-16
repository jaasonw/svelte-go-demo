import type { RequestEvent } from "../api/$types";

const multiplyMatrices = (a: any, b: any) => {
  if (!Array.isArray(a) || !Array.isArray(b) || !a.length || !b.length) {
    throw new Error("arguments should be in 2-dimensional array format");
  }
  let x = a.length,
    z = a[0].length,
    y = b[0].length;
  if (b.length !== z) {
    // XxZ & ZxY => XxY
    throw new Error(
      "number of columns in the first matrix should be the same as the number of rows in the second"
    );
  }
  let productRow = Array.apply(null, new Array(y)).map(
    Number.prototype.valueOf,
    0
  );
  let product = new Array(x);
  for (let p = 0; p < x; p++) {
    product[p] = productRow.slice();
  }
  for (let i = 0; i < x; i++) {
    for (let j = 0; j < y; j++) {
      for (let k = 0; k < z; k++) {
        product[i][j] += a[i][k] * b[k][j];
      }
    }
  }
  return product;
};

export function GET({ url }: RequestEvent) {
  const matrix_size = 300;

  const matrix_a = Array.from({ length: matrix_size }, () =>
    Array.from({ length: matrix_size }, () => Math.floor(Math.random() * 10))
  );
  const matrix_b = Array.from({ length: matrix_size }, () =>
    Array.from({ length: matrix_size }, () => Math.floor(Math.random() * 10))
  );

  const t0 = performance.now();
  multiplyMatrices(matrix_a, matrix_b);
  const t1 = performance.now();

  return new Response(`${t1 - t0} ms`);
}
