def go(n):
    xx=[[set(), False] for i in range(n)]   
    k,r,c=0,0,0
    for y in range(n):
        rows=(int(x) for x in input().split(' '))
        yy=set()
        ydup=False
        for x,val in enumerate(rows):
            if y==x: k+=val
            if val in yy: 
                r+=1 if not ydup else 0
                ydup=True
            else:
                yy.add(val)
            if val in xx[x][0]:
                c+=1 if not xx[x][1] else 0
                xx[x][1]=True
            else:
                xx[x][0].add(val)
    return k,r,c

t=int(input())
for i in range(t):
    n=int(input())
    k,r,c=go(n)
    print('Case #{0}: {1} {2} {3}'.format(i+1,k,r,c))