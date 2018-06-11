import { Automaton, automatonGapMatcher, finiteAutomatonMatcher } from "./automaton";
import { computePrefixFunction, kmpMatcher } from "./kmp";
import { gapStringMatcher, match, naiveStringMatcher } from "./naive";
import { rabinKarpMatcher } from "./rabin-karp";

function main() {
  matchingTest();
}

function matchingTest() {
  let T = `Text-editing programs frequently need to find all occurrences of a pattern in the text.
Typically, the text is a document being edited, and the pattern searched for is a particular word supplied by the user.
Efficient algorithms for this problem—called “string matching”—can greatly aid the responsiveness of the text-editing
program. Among their many other applications, string-matching algorithms search for particular patterns in DNA
sequences. Internet search engines also use them to find Web pages relevant to queries`;
  let P = "the";

  console.log("Naive:");
  console.log(naiveStringMatcher(T, P));
  console.log("Rabin-Karp");
  console.log(rabinKarpMatcher(T, P));
  console.log("Finite automaton:");
  console.log(finiteAutomatonMatcher(T, P));
  console.log("Knuth-Morris-Pratt:");
  console.log(kmpMatcher(T, P));
}

function problem_32_1_1() {
  let T = "000010001010001";
  let P = "0001";

  console.log(naiveStringMatcher(T, P));
  console.log(rabinKarpMatcher(T, P));
}

function problem_32_1_4() {
  let T = "000010001010001";
  let P1 = ["01", "01", "01", "01"];
  let P2 = ["01", "01", "01", "01", "01"];

  console.assert(gapStringMatcher(T, P1) === true);
  console.assert(gapStringMatcher(T, P2) === false);
  //  problem_32_3_5
  console.assert(automatonGapMatcher(T, P1) === true);
  console.assert(automatonGapMatcher(T, P2) === false);
}

function problem_32_2_1() {
  let T = "3141592653589793";
  let P = "26";
  let q = 11;
  let p = parseInt(P, 10) % q;
  let n = T.length;
  let m = P.length;

  let cnt = 0;
  for (let i = 0; i <= n - m; i++) {
    let t = parseInt(T.substr(i, m), 10);
    if (t % q === p && !match(T, P, i)) {
      console.log(`spurious hit at offset ${i}: t = ${t}, t mod q = ${t % q} = p mod q`);
      cnt++;
    }
  }
  console.log(cnt);
}

function problem_32_3_1() {
  let T = "aaababaabaababaab";
  let P = "aabab";

  console.log(finiteAutomatonMatcher(T, P));
  console.log(naiveStringMatcher(T, P));
  console.log(kmpMatcher(T, P));
}

function problem_32_3_2() {
  let P = "ababbabbababbababbabb";
  let M = new Automaton(P);

  M.print();
}

function problem_32_4_1() {
  console.log(computePrefixFunction("ababbabbabbababbabb"));
}

main();
