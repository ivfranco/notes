export {
  rabinKarpMatcher,
};

import { mod, modExp } from "../number/modular";
import { match } from "./naive";

//  d = 65536 = 2^16 for javscript character codes
const DIGIT = 2 ** 16;
const MODULO = Math.floor(Number.MAX_SAFE_INTEGER / DIGIT);

function rabinKarpMatcher(T: string, P: string): number[] {
  let n = T.length;
  let m = P.length;
  let h = modExp(DIGIT, m - 1, MODULO);

  let p = 0;
  let t = 0;
  for (let i = 0; i < m; i++) {
    p = mod(DIGIT * p + P.charCodeAt(i), MODULO);
    t = mod(DIGIT * t + T.charCodeAt(i), MODULO);
  }

  let shifts: number[] = [];
  for (let s = 0; s <= n - m; s++) {
    if (p === t && match(T, P, s)) {
      shifts.push(s);
    }
    //  used to be an one liner
    //    t = mod(DIGIT * (t - T.charCodeAt(s) * h) + T.charCodeAt(s + m), MODULO);
    //  but the above operation may overflow Number.MAX_SAFE_INTEGER
    t = mod(DIGIT * (t - T.charCodeAt(s) * h), MODULO);
    t = mod(t + T.charCodeAt(s + m), MODULO);
  }

  return shifts;
}
