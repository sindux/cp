#%%
from collections import Counter
def do(s, repls):
    INF = float('inf')
    chmap = {r[0]+r[1]:1 for r in repls}
    vs = set(c for cp in chmap for c in cp)
    for k in vs:
        for i in vs:
            for j in vs:
                if chmap.get(i+j, INF) > chmap.get(i+k,INF) + chmap.get(k+j, INF):
                    chmap[i+j] = chmap[i+k] + chmap[k+j]
    # print(vs, chmap)

    c = Counter(s)
    ans = float('inf')
    for ch in list(vs) + list(c):
        t=0
        for ch2,v2 in c.items():
            if ch2==ch: continue
            d = chmap.get(ch2+ch, INF)
            if d == INF:
                t=INF
                break
            else:
                t+=d*v2
        # print(ch,t)
        ans=min(ans,t)
    return ans if ans != INF else -1

for t in range(int(input())):
    s = input()
    k = int(input())
    repls = [input() for _ in range(k)]
    a = do(s, repls)
    print(f'Case #{t+1}: {a}')

