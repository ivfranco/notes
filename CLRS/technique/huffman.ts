export {
  huffman,
};

import { TreeNode } from "../collection/tree";
import { PriorityQueue } from "../sort/heap";

type Freq = number;

class HuffmanNode extends TreeNode<Freq> {
  public char: string | null;

  constructor(freq: Freq, char: string | null) {
    super(freq);
    this.char = char;
  }

  public freq(): number {
    return this.key;
  }

  protected nodeStringify(): string {
    if (this.char) {
      return `${this.char}:${this.freq()}`;
    } else {
      return `${this.freq()}`;
    }
  }
}

class HuffmanPQ extends PriorityQueue<HuffmanNode> {
  public cmp(a: HuffmanNode, b: HuffmanNode): boolean {
    return a.freq() < b.freq();
  }
}

function huffman(C: Array<[Freq, string]>): HuffmanNode {
  let n = C.length;
  let q = new HuffmanPQ(C.map(([f, c]) => new HuffmanNode(f, c)));

  //  i counts the number of nodes in the min priority queue
  for (let i = n; i > 1; i--) {
    let z = new HuffmanNode(0, null);
    let x = q.extractRoot();
    let y = q.extractRoot();
    z.left = x;
    z.right = y;
    z.key = x.freq() + y.freq();
    q.insertKey(z);
  }

  return q.extractRoot();
}
