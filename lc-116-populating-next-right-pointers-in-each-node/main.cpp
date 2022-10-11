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
    queue<Node*> q;
    int i = 0;
    q.push(root);
    while (!q.empty()) {
      auto cnt = (1 << i) - 1;
      auto node = q.front();
      if (node == nullptr)
        break;
      q.pop();
      q.push(node->left);
      q.push(node->right);
      while (cnt-- > 0) {
        auto next = q.front();
        q.pop();
        node->next = next;
        node = next;
        q.push(node->left);
        q.push(node->right);
      }
      node->next = nullptr;
      i++;
    }
    return root;
  }
};

int main() {
  return 0;
}