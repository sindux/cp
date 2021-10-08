#%%
# X  F    O
# 0  0/1  1

def go(s):
    # print(s)
    m = [10**9, 10**9]
    for c in range(len(s)):
        ch=s[c]
        if c==0:
            m[0] = 0 if ch in 'XF' else 10**9
            m[1] = 0 if ch in 'FO' else 10**9
        else:
            if ch == 'X':
                m=[min(m[0], m[1]+1), 10**9]
            elif ch == 'O':
                m=[10**9, min(m[1], m[0]+1)]
            else:
                m = [min(m[0], m[1]+1),
                     min(m[1], m[0]+1)]
            
        # print(c,ch,m)
    return min(m)

for t in range(int(input())):
    _ = input()
    ans = go(input())
    print(f'Case #{t+1}: {ans}')

