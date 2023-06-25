#include <cstdio>
#include <iostream>

using namespace std;

const int MAXN = 1e5, MAXM = MAXN;

struct Node
{
  int id;  // -1 if removed
  int l;   // -1 if first
  int r;   // -1 if last
} nodes[MAXN];

int root = 0, tail, n;

void remove(int i)
{
  // check root
  if (i == root)
  {
    root = nodes[i].r;
    nodes[root].l = -1;
  }
  else if (i == tail)
  {
    tail = nodes[i].l;
    nodes[tail].r = -1;
  }
  else if (nodes[i].l == nodes[i].r && nodes[i].r == -1)
  {
    // already removed
    return;
  }
  else
  {
    int left = nodes[i].l;
    int right = nodes[i].r;
    nodes[left].r = nodes[right].id;
    nodes[right].l = nodes[left].id;
  }
  nodes[i].l = nodes[i].r = -1;
}

void insert(int i, int k, int p)
{
  if (k == root && p == 0)
  {
    // insert i at the left of k
    nodes[root].l = i;
    nodes[i].r = root;
    nodes[i].l = -1;
    root = i;
  }
  else if (k == tail && p == 1)
  {
    // insert i at the right of k
    nodes[tail].r = i;
    nodes[i].l = tail;
    nodes[i].r = -1;
    tail = i;
  }
  else
  {
    int left = p == 0 ? nodes[k].l : k;
    int right = p == 0 ? k : nodes[k].r;
    nodes[left].r = i;
    nodes[right].l = i;
    nodes[i].l = left;
    nodes[i].r = right;
  }
}

void move(int i, int k, int p)
{
  // p == 0 => insert i to the left of k
  // p == 1 => insert i to the right of k

  // First, remove i from the list
  remove(i);
  // Then, insert i
  insert(i, k, p);
}

void print()
{
  for (int i = 0; i < n; i++)
  {
    printf("node %d, l = %d, r = %d\n", nodes[i].id, nodes[i].l, nodes[i].r);
  }
}

int main()
{
  int k, p, m;
  nodes[0] = { 0, -1, 1 };
  scanf("%d", &n);
  nodes[n - 1] = { n - 1, n - 2, -1 };
  for (int i = 1; i < n - 1; i++)
  {
    nodes[i].id = i;
    nodes[i].r = i + 1;
    nodes[i].l = i - 1;
  }
  tail = n - 1;
  for (int i = 1; i < n; i++)
  {
    scanf("%d %d", &k, &p);
    move(i, k - 1, p);
  }
  scanf("%d", &m);
  for (int i = 0; i < m; i++)
  {
    scanf("%d", &k);
    remove(k - 1);
  }
  for (int i = root; i != -1;)
  {
    int id = nodes[i].id + 1;
    i = nodes[i].r;
    printf("%d", id);
    if (i != -1)
      putchar(' ');
  }
  return 0;
}