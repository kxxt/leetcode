#include <cstdio>
#include <climits>
#include <iostream>
#include <cmath>

using namespace std;

const int MAXN = 200000;

int n, dp[MAXN + 1], nums[MAXN];

// >>> 2e5 * 1e4 < 2147483647
// True

int main()
{
  scanf("%d", &n);
  scanf("%d", &nums[0]);
  if (n == 1)
  {
    printf("%d\n", nums[0]);
    return 0;
  }
  dp[0] = nums[0];
  int maxdp = dp[0];
  for (int i = 1; i < n; i++)
  {
    scanf("%d", &nums[i]);
    dp[i] = max(dp[i - 1] + nums[i], nums[i]);
    maxdp = max(maxdp, dp[i]);
  }
  printf("%d\n", maxdp);
  return 0;
}