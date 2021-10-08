#%%
from collections import Counter, defaultdict
def do(b):
    rows=[Counter(r) for r in b]
    cols=[Counter(r[c] for r in b) for c in range(len(b))]
    ans=defaultdict(set)
    for r,row in enumerate(rows):
        if not 'O' in row:
            ans[row['.']].add(tuple(sorted((r,ii) for ii,c in enumerate(b[r]) if c=='.')))
    for c,col in enumerate(cols):
        if not 'O' in col:
            ans[col['.']].add(tuple(sorted((ii,c) for ii in range(len(b)) if b[ii][c]=='.')))
    #print(ans)
    if ans:
        minx = sorted(ans.keys())[0]
        return '{} {}'.format(minx,len(ans[minx]))
    return 'Impossible'

for t in range(int(input())):
    n = int(input())
    boards = [input() for _ in range(n)]
    a = do(boards)
    print(f'Case #{t+1}: {a}')

