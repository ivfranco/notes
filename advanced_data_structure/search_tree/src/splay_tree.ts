export { SplayTree, SplayNode };

import { TreeLike } from "./lib";
import { Comparator, Ordering } from "./comparator";

enum Branch {
  Left,
  Right,
}

class SplayNode<K, V> {
  key: K;
  value: V;
  parent: SplayNode<K, V> | null = null;
  // both left and right child of a model 2 tree node can be null
  left_child: SplayNode<K, V> | null = null;
  right_child: SplayNode<K, V> | null = null;

  constructor(key: K, value: V) {
    this.key = key;
    this.value = value;
  }
}

function connect_left<K, V>(parent: SplayNode<K, V>, child: SplayNode<K, V> | null) {
  parent.left_child = child;
  if (child) {
    child.parent = parent;
  }
}

function connect_right<K, V>(parent: SplayNode<K, V>, child: SplayNode<K, V> | null) {
  parent.right_child = child;
  if (child) {
    child.parent = parent;
  }
}

function minimum_node<K, V>(node: SplayNode<K, V>): SplayNode<K, V> {
  while (node.left_child) {
    node = node.left_child;
  }

  return node;
}

function left_rotation<K, V>(node: SplayNode<K, V>) {
  let right_child = node.right_child;
  if (right_child == null) {
    return;
  }

  connect_right(node, right_child.right_child);
  connect_right(right_child, right_child.left_child);
  connect_left(right_child, node.left_child);
  connect_left(node, right_child);

  let temp_key = node.key;
  node.key = right_child.key;
  right_child.key = temp_key;

  let temp_value = node.value;
  node.value = right_child.value;
  right_child.value = temp_value;
}

function right_rotation<K, V>(node: SplayNode<K, V>) {
  let left_child = node.left_child;
  if (left_child == null) {
    return;
  }

  connect_left(node, left_child.left_child);
  connect_left(left_child, left_child.right_child);
  connect_right(left_child, node.right_child);
  connect_right(node, left_child);

  let temp_key = node.key;
  node.key = left_child.key;
  left_child.key = temp_key;

  let temp_value = node.value;
  node.value = left_child.value;
  left_child.value = temp_value;
}

class SplayTree<K, V> implements TreeLike<K, V> {
  cmp: Comparator<K>;
  root: SplayNode<K, V> | null = null;

  constructor(cmp: Comparator<K>) {
    this.cmp = cmp;
  }

  find(key: K): V | null {
    let node = this.narrow_to_node(key);
    if (node == null) {
      return null;
    }

    while (node.parent != null) {
      let upper: SplayNode<K, V> = node.parent;

      if (upper.parent == null) {
        if (node == upper.left_child) {
          // case 2.1
          right_rotation(upper);
        } else {
          // case 2.2
          left_rotation(upper);
        }
        node = upper;
      } else {
        let top = upper.parent;

        if (node == upper.left_child && upper == top.left_child) {
          // case 3.1
          right_rotation(top);
          right_rotation(top);
        } else if (node == upper.left_child && upper == top.right_child) {
          // case 3.2
          right_rotation(upper);
          left_rotation(top);
        } else if (node == upper.right_child && upper == top.left_child) {
          // case 3.3
          left_rotation(upper);
          right_rotation(top);
        } else {
          // case 3.4
          left_rotation(top);
          left_rotation(top);
        }

        node = top;
      }
    }

    return node.value;
  }

  // insert (key, value) pair, replace old value when key exists
  insert(key: K, value: V) {
    let new_leaf = new SplayNode(key, value);
    let upper = null;
    let node = this.root;

    if (node == null) {
      this.root = new_leaf;
      return;
    }

    let branch!: Branch;

    // the loop is executed at least once, `branch` and `upper` are assigned
    while (node) {
      upper = node;
      if (this.cmp(key, upper.key) == Ordering.LT) {
        branch = Branch.Left;
        node = upper.left_child;
      } else if (this.cmp(key, upper.key) == Ordering.GT) {
        branch = Branch.Right;
        node = upper.right_child;
      } else {
        upper.value = value;
        return;
      }
    }

    if (branch == Branch.Left) {
      connect_left(upper!, new_leaf);
    } else {
      connect_right(upper!, new_leaf);
    }
  }

  narrow_to_node(search_key: K): SplayNode<K, V> | null {
    let node = this.root;

    while (node) {
      if (this.cmp(search_key, node.key) == Ordering.EQ) {
        break;
      } else if (this.cmp(search_key, node.key) == Ordering.LT) {
        node = node.left_child;
      } else {
        node = node.right_child;
      }
    }

    return node;
  }

  // CLRS, Chapter 12.3
  delete(key: K): V | null {
    let deleted = this.narrow_to_node(key);
    if (deleted == null) {
      return null;
    }

    if (deleted.left_child == null) {
      this.transplant(deleted, deleted.right_child);
    } else if (deleted.right_child == null) {
      this.transplant(deleted, deleted.left_child);
    } else {
      let right_min = minimum_node(deleted.right_child);
      if (right_min.parent != deleted) {
        this.transplant(right_min, right_min.right_child);
        connect_right(right_min, deleted.right_child);
      }
      this.transplant(deleted, right_min);
      connect_left(right_min, deleted.left_child);
    }

    return deleted.value;
  }

  transplant(old_child: SplayNode<K, V>, new_child: SplayNode<K, V> | null) {
    if (old_child.parent == null) {
      // `old_child` is root
      this.root = new_child;
      if (new_child) {
        new_child.parent = null;
      }
    } else if (old_child == old_child.parent.left_child) {
      connect_left(old_child.parent, new_child);
    } else {
      connect_right(old_child.parent, new_child);
    }
  }

  to_dot(): string {
    if (this.root) {
      return to_dot(this.root);
    } else {
      return "Empty tree";
    }
  }
}

function to_dot<K, V>(root: SplayNode<K, V>): string {
  let last_index = 0;
  let stack: [SplayNode<K, V>, number][] = [[root, last_index]];
  let dot = "";

  while (stack.length > 0) {
    let [node, index] = stack.pop()!;
    dot += `node${index} [label = "${node.key}, ${node.value}"]\n`;

    if (node.left_child) {
      let left_child = node.left_child;
      dot += `node${index} -> node${last_index + 1}\n`;
      stack.push([left_child, last_index + 1]);
      last_index += 1;
    }

    if (node.right_child) {
      let right_child = node.right_child;
      dot += `node${index} -> node${last_index + 1}\n`;
      stack.push([right_child, last_index + 1]);
      last_index += 1;
    }
  }

  return `Digraph G {
    ${dot}
  }`;
}
