def go(ds):
    ans=0
    ds=sorted(ds)
    for i in ds:
        if i>=ans+1:
            ans+=1
    return ans


tt=int(input())
for t in range(tt):
    input()
    ds = list(map(int, input().split()))
    ans=go(ds)
    print(f'Case #{t+1}: {ans}')

