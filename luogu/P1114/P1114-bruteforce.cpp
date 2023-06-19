#include <cstdio>
#include <iostream>
#include <cmath>

const int MAXN = 100000;

int n, nums[MAXN], sums[MAXN + 1];

int main()
{
  // Read input
  scanf("%d", &n);
  sums[0] = 0;
  for (int i = 0; i < n; i++)
  {
    scanf("%d", &nums[i]);
    // prefix sum
    sums[i + 1] = sums[i] + nums[i];
  }
  // calc ans
  int ans = 0;
  for (int i = 0; i < n; i++)
  {
    for (int j = i + 1; j < n; j += 2)  // odd len is not possible
    {
      auto len = j - i + 1;
      auto cnt = sums[j + 1] - sums[i];
      if (2 * cnt == len)
        ans = std::max(len, ans);
    }
  }
  printf("%d\n", ans);
  return 0;
}