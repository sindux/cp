def go(r,c):
    ans=[]
    l=['..+'] + ['-+'] * (c-1)
    ans.append(''.join(l))
    for rr in range(r):
        l=['..|' if rr==0 else '|.|'] + ['.|'] * (c-1)
        ans.append(''.join(l))
        
        l=['+'] + ['-+'] * c 
        ans.append(''.join(l))
    return '\n'.join(ans)

t=int(input())
for tt in range(t):
    r,c=list(map(int, input().split()))
    ans=go(r,c)
    print(f'Case #{tt+1}:')
    print(ans)
    

            