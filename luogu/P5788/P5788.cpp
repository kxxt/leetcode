#include <cstdio>
#include <iostream>

using namespace std;

const int MAXN = 3e6;

int n, num, idx[MAXN], f[MAXN], stack[MAXN], top = 0;

int main()
{
  scanf("%d", &n);
  for (int i = 0; i < n; i++)
  {
    scanf("%d", &num);
    while (top && stack[top - 1] < num)
    {
      // record this
      f[idx[top - 1]] = i + 1;
      --top;
    }
    idx[top] = i;
    stack[top++] = num;
  }
  for (int i = 0; i < n; i++)
  {
    printf("%d", f[i]);
    if (i != n - 1)
      putchar(' ');
  }
  return 0;
}