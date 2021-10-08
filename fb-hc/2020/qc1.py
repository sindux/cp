#%%
import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import floyd_warshall

def do(n, inb, outb):
    inb=np.array([inb]).view('U1') == 'N'
    outb=np.array([outb]).view('U1') == 'N'
    idxs=np.arange(n)
    dist=np.identity(n, np.int32)
    dist[idxs[:-1],idxs[1:]]=1
    dist[idxs[1:],idxs[:-1]]=1
    dist[outb,:]=0
    dist[:,inb]=0
    dist[idxs, idxs]=1

    #print(dist)
    g = csr_matrix(dist)
    res = floyd_warshall(g)
    return np.array(['Y', 'N'])[np.isinf(res)*1].view(f'U{n}').ravel()

#res=do(5, 'YNNYY','YYYNY')
#print('\n'.join(ans))

#%%
t = int(input())
for tt in range(t):
    n = int(input())
    inb = input()
    outb = input()
    ans = do(n, inb, outb)
    print(f'Case #{tt+1}:')
    print('\n'.join(ans))
