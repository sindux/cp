def go(n, k, v, a):
    i=(v-1)*k % n
    for _ in range(k):
        yield i, a[i]
        i=(i+1) % n

t=int(input())
for i in range(t):
    n,k,v=map(int, input().split())
    a=[input() for _ in range(n)]
    res=list(go(n,k,v,a))
    res=sorted(res, key=lambda x:x[0])
    print(f'Case #{i + 1}: {" ".join(map(lambda x:x[1], res))}')