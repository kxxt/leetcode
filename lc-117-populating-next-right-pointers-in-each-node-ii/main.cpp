#include <iostream>
#include <queue>

class Node {
 public:
  int val;
  Node* left;
  Node* right;
  Node* next;

  Node() : val(0), left(NULL), right(NULL), next(NULL) {}

  Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

  Node(int _val, Node* _left, Node* _right, Node* _next)
      : val(_val), left(_left), right(_right), next(_next) {}
};

class Solution {
 public:
  Node* connect(Node* root) {
    using namespace std;
    if (!root)
      return root;
    queue<Node*> q;
    q.push(root);
    while (!q.empty()) {
      auto i = q.size();
      Node* right = nullptr;
      while (i-- > 0) {
        auto node = q.front();
        q.pop();
        node->next = right;
        if (node->right)
          q.push(node->right);
        if (node->left)
          q.push(node->left);
        right = node;
      }
    }
    return root;
  }
};

int main() {
  return 0;
}