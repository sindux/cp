def flip(state, idx, k):
    if idx >= len(state) - k:
        rem = state[-k:]
        return 0 if rem == [1] * k else 1 if rem == [0] * k else -1
    try:
        f = state.index(0, idx, len(state)-k)
        for i in range(k):
            state[f+i] = 1-state[f+i]
        nxt = flip(state, f+1, k)
        ans = (1 + nxt) if nxt >= 0 else -1
        for i in range(k):
            state[f+i] = 1-state[f+i]
        return ans
    except ValueError:
        return flip(state, len(state)-k, k)
import sys
sys.setrecursionlimit(1500)
t = int(input())
for tt in range(t):
    state, k = input().split()
    state = list(map(lambda x: 1 if x == '+' else 0, state))
    k = int(k)
    ans = flip(state, 0, k)
    print('Case #{}: {}'.format(tt + 1, ans if ans >= 0 else 'IMPOSSIBLE'))
