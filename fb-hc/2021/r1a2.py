#%%
# X  F    O
# 0  0/1  1
import numpy as np
def go(s):
    # print(s)
    ans=0
    m = [[10**9, 10**9] for _ in range(len(s))]
    for c in range(len(s)):
        ch=s[c]
        if c==0:
            m[c][0] = 0 if ch in 'XF' else 10**9
            m[c][1] = 0 if ch in 'FO' else 10**9
        else:
            if ch == 'X':
                m[c]=[min(m[c-1][0], m[c-1][1]+1), 10**9]
            elif ch == 'O':
                m[c]=[10**9, min(m[c-1][1], m[c-1][0]+1)]
            else:
                m[c] = [min(m[c-1][0], m[c-1][1]+1),
                        min(m[c-1][1], m[c-1][0]+1)]
        ans += min(m[c])    
        # print(c,ch,m)
    print(s)
    print(np.array(m))
    return ans % 1_000_000_007

for t in range(int(input())):
    _ = input()
    s = input()
    ans=0
    for ss in range(len(s)):
        ans += go(s[ss:])
    print(f'Case #{t+1}: {ans%(10**9+7)}')

