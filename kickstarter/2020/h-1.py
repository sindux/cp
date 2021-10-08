def do(n,k,s):
    return min(k-1+k-s+n-s+1, k-1+1+n)

for t in range(int(input())):
    n,k,s=list(map(int, input().split()))
    res=do(n,k,s)
    print('Case #{}: {}'.format(t+1, res))
