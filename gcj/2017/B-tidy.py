def go(n, idx, tidy, ismax, prev):
    #print(locals())
    if idx >= len(n):
        return tidy
    ans = -1
    tryfrom = 9
    if ismax:
        cur = int(n[idx])
        if cur >= prev:
            ans = go(n, idx + 1, tidy * 10 + cur, ismax, cur)
            if ans == -1:
                tryfrom = cur - 1
        else:  # no point trying max
            return -1
    if ans == -1:
        for totry in range(tryfrom, prev - 1, -1):
            ans = go(n, idx + 1, tidy * 10 + totry, False, totry)
            if ans >= 0:
                return ans
    return ans


t=int(input())
for tt in range(t):
    n=input()
    print('Case #{}: {}'.format(tt+1,go(n, 0, 0, True, 0)))
