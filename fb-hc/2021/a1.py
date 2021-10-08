#%%
from collections import Counter
def a1(s):
    vow = 'AIUEO'
    c = Counter(s)
    ans = len(s)*2
    if all(ch in vow for ch in c) or all(ch not in vow for ch in c):
        ans = len(s)
    for ch,v in c.items():
        t=0
        isvowel = ch in vow
        for ch2,v2 in c.items():
            if ch2==ch: continue
            isvowel2 = ch2 in vow
            if isvowel==isvowel2: t+=v2*2
            else: t+=v2
        ans=min(ans,t)
    return ans

for t in range(int(input())):
    a = a1(input())
    print(f'Case #{t+1}: {a}')

