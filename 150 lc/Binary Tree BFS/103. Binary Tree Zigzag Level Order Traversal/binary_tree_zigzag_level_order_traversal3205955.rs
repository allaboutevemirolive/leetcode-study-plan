// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3205955/clean-bfs-solution-rust-javascript/
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
const zigzagLevelOrder = function (root) {
  if (!root) return [];
  const queue = [root];
  const values = [];

  while (queue.length > 0) {
    const length = queue.length;
    const level = [];

    for (let i = 0; i < length; i++) {
      const node = queue.shift();

      level.push(node.val);
      node.left && queue.push(node.left);
      node.right && queue.push(node.right);
    }
    if (values.length & 1) {
      level.reverse();
    }
    values.push(level);
  }

  return values;
};