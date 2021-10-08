#%%
import numpy as np
from itertools import permutations

# def go(xs):
#     ans=0
#     for i in range(len(xs)-1):
#         j=np.argmin(xs[i:])+i
#         ans+=j-i+1
#         xs[i:j+1]=np.flip(xs[i:j+1])
#         yield j-i+1
#     yield ans

# for i in permutations(range(1,5)):
#     print(i, list(go(np.array(i))))

import numpy as np
def go(xs):
    ans=0
    for i in range(len(xs)-1):
        j=np.argmin(xs[i:])+i
        ans+=j-i+1
        xs[i:j+1]=np.flip(xs[i:j+1])
    return ans

def go2(n,c):
    for i in permutations(range(1,n+1)):
        if go(np.array(i))==c: return list(i)
    return 'IMPOSSIBLE'
    
for t in range(int(input())):
    n,c = [int(i) for i in input().split()]
    print('Case #{}: {}'.format(t+1, go2(n,c)))