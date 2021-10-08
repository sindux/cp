def parsep(n, p):
    res={}
    pos=0
    for dir in p:
        res[pos]=dir
        pos += 1 if dir=='E' else n
    return res

def go(n, p, final, pos=0, res=[], visited=set()):
    if pos==final:
        return True
    if pos in visited:
        return False
    visited.add(pos)
    blocked=p.get(pos, ' ')
    y=pos//n
    x=pos%n
    if x>=y:
        if pos < n*n-n and blocked !='S':
            res.append('S')
            ok=go(n,p,final,pos+n,res,visited)
            if ok:
                return ok
            res.pop()
        if (pos+1)%n != 0 and blocked != 'E':
            res.append('E')
            ok=go(n, p, final, pos+1, res, visited)
            if ok:
                return ok
            res.pop()
    else:
        if (pos+1)%n != 0 and blocked != 'E':
            res.append('E')
            ok=go(n, p, final, pos+1, res, visited)
            if ok:
                return ok
            res.pop()
        if pos < n*n-n and blocked !='S':
            res.append('S')
            ok=go(n,p,final,pos+n,res,visited)
            if ok:
                return ok
            res.pop()

def main():
    for i in range(int(input())):
        n = int(input())
        paths=parsep(n, input())
        #ans=ii(n,go(n, paths))
        ans=[]
        go(n,paths,n*n-1,0,ans,set())
        print('Case #{}: {}'.format(i+1, ''.join(ans)))

import sys
sys.setrecursionlimit(100001)
main()
#ans=[]
#n=1000
#go(n,parsep(n,'EESSSESE'),n*n-1,0,ans,set())
#print(ans)

#ii(5,go(5,parsep(5,'EESSSESE')))

