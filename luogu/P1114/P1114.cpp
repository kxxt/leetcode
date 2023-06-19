#include <cstdio>
#include <iostream>
#include <cmath>

const int MAXN = 100010;

int n, nums[MAXN], sum[MAXN + 1], loc[2 * MAXN + 2][2], cnt = 0;

int main()
{
  using namespace std;
  // Read input
  scanf("%d", &n);
  if (n == 1)
  {
    puts("0");
    return 0;
  }

  for (int i = -n; i <= n; i++)
  {
    loc[MAXN + i][0] = MAXN;
    loc[MAXN + i][1] = -1;
  }
  loc[MAXN + 0][0] = loc[MAXN + 0][1] = 0;
  for (int i = 0; i < n; i++)
  {
    scanf("%d", &nums[i]);
    if (0 == nums[i])
      nums[i] = -1;
    sum[i + 1] = sum[i] + nums[i];
    loc[MAXN + sum[i + 1]][0] = min(loc[MAXN + sum[i + 1]][0], i + 1);
    loc[MAXN + sum[i + 1]][1] = max(loc[MAXN + sum[i + 1]][1], i + 1);
  }
  int ans = 0;
  for (int i = -n - 1; i <= n + 1; i++)
  {
    // cout << "loc[" << i << "] = " << loc[MAXN + i][0] << ", " << loc[MAXN + i][1] << endl;
    if (loc[MAXN + i][0] < MAXN && loc[MAXN + i][1] >= 0)
    {
      ans = max(ans, loc[MAXN + i][1] - loc[MAXN + i][0]);
    }
  }
  printf("%d\n", ans);
  return 0;
}