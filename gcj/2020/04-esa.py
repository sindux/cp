import sys
def do(n):
    ans=[' '] * n
    for i in range(150):
        print(i % n + 1, flush=True)
#        print(i % n + 1, file=sys.stderr, flush=True)
        b=input()
        ans[i % n]=b
    return ''.join(ans)
    
t,b=list(map(int, input().split(' ')))
for i in range(t):
    ans=do(b)
    print(ans, flush=True)
    judge=input()
    #print(ans + ' ' + judge, flush=True, file=sys.stderr)
    
    if judge=='N':
        break
        