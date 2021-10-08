def do(evs):
    evs=sorted(evs, key=lambda a:(a[1],a[2]))
    #print(evs)
    c=[-1,-1]
    j=[-1,-1]
    ans=[]
    for ev in evs:
        if ev[1]>=c[1]:
            ans.append('C')
            c=ev[1:]
            continue
        if ev[1]>=j[1]:
            ans.append('J')
            j=ev[1:]
            continue
        return 'IMPOSSIBLE'
    ansch = [' '] * len(evs)
    for idx,who in enumerate(ans):
        ansch[evs[idx][0]]=who
    return ansch

t=int(input())
for i in range(t):
    n=int(input())
    evs=[[act]+list(map(int, input().split(' '))) for act in range(n)]
    ans=do(evs)
    print('Case #{0}: {1}'.format(i+1, ''.join(ans)))
