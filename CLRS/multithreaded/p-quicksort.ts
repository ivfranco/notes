import { partition, randomPivoter } from "../sort/quicksort";

async function pRandomizedQuicksort<T>(A: T[], p: number, r: number) {
  let q = partition(A, p, r, randomPivoter);
  let handle = pRandomizedQuicksort(A, p, q - 1);
  await pRandomizedQuicksort(A, q + 1, r);
  await handle;
}
