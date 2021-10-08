from typing import List

def k(s:str):
    i=s.index('*')
    return i

def brk(s:str) -> List[str]:
    ss=s.split('*')
    ss2=[]
    if ss[0]!='': ss2.append(ss[0])
    for i in ss[1:]:
        if i!='':
            ss2.append('')
        ss2.append(i)
    return ss2

def ff(l:List[str], st: int) -> int:
    for idx,i in enumerate(l[st:]):
        if i!='': return idx+st
    return -1 

def fit(ans:List[str],ps:List[str]):
    ans=ans.copy()
    a=ff(ans,0)
    p=ff(ps,0)
    while (a>=0 and p>=0):
        ok=False
        if ans[a] in ps[p]:
            ans[a]=ps[p]
            ok=True
        elif ps[p] in ans[a]:
            ok=True
        # if a>0 and ans[a-1]=='':
        #     ans[a-1]+=ps[p]
        #     ok=True
        if p>0 and ps[p-1]=='':
            if a-2>=0 and ps[p] in ans[a-2]: 
                ok=True
                break
            if not ok:
                ta=a
                while ta>=0:
                    if ans[ta]!='':
                        if ta < len(ans)-1:
                            ans[ta]=ans[ta]+ps[p]
                            ok=True
                        break
                    ta-=1
        if not ok:
            return None
        a=ff(ans,a+1)
        p=ff(ps,p+1)
    if a>=0 and p<0:
        if ps[-1]!='':
            return None
    if a<0 and p>=0:
        if ans[-1]!='':
            return None
    return ans

def go(ps:List[str]):
    ps=sorted(ps,key=k,reverse=True)
    ans=brk(ps[0])
    for p in ps[1:]:
        ll=brk(p)
        nans=fit(ans,ll)
        if not nans:
            return '*'
        ans=nans
    return ''.join(nans)


def c(s):
    return s.replace('**','*')

t=int(input())
for i in range(t):
    n=int(input())
    pats=[c(input()) for _ in range(n)]
    cans=go(pats)
    print('Case #{0}: {1}'.format(i+1,cans))