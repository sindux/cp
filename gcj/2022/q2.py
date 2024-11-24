def go(inks):
    mi = [10**6] * 4 
    for i in inks:
        for c in range(4):
            mi[c]=min(mi[c], i[c])
    if sum(mi)<10**6: return 'IMPOSSIBLE'
    c=0
    for i in range(4):
        totake = min(mi[i], 10**6-c)
        mi[i]=totake
        c+=totake
#    print(sum(mi))
    return ' '.join(map(str, mi))


tt=int(input())
for t in range(tt):
    p=[]
    for i in range(3):
        p.append(list(map(int, input().split())))
    ans=go(p)
    print(f'Case #{t+1}: {ans}')

