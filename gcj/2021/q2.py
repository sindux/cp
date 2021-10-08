# from functools import lru_cache

# class Sol:
#     def __init__(self,x,y,xs):
#         self.go.cache_clear()
#         self.x,self.y,self.xs=x,y,xs
#     def do(self):
#         if self.xs[-1]==-1:
#             return min(self.go(len(xs)-1,0),
#                        self.go(len(xs)-1,1))
#         else:
#             return self.go(len(xs)-1,self.xs[-1])

#     @lru_cache(maxsize=None)
#     def go(self,idx,cj):
#         if idx==0: return 0
#         if self.xs[idx-1]==-1:
#             p0=self.go(idx-1,0)
#             if cj==1: p0+=self.x
#             p1=self.go(idx-1,1)
#             if cj==0: p1+=self.y
#             return min(p0,p1)
#         else:
#             p=self.go(idx-1, self.xs[idx-1])
#             if self.xs[idx-1]==0 and cj==1: p+=self.x
#             elif self.xs[idx-1]==1 and cj==0: p+=self.y
#             return p
            
    
# for t in range(int(input())):
#     x,y,xs=input().split()
#     ans=Sol(int(x),int(y),[{'C':0, 'J':1, '?': -1}[x] for x in xs]).do()
#     print('Case #{}: {}'.format(t+1, ans))

def go(x,y,xs):
    c=[0,0]
    for i in range(1,len(xs)):
        nc=[0,0]
        if xs[i-1]==-1:
            nc[0]=min(c[0],c[1]+y)
            nc[1]=min(c[1],c[0]+x)
        elif xs[i-1]==0:
            nc[0]=c[0]
            nc[1]=c[0]+x
        else:
            nc[0]=c[1]+y
            nc[1]=c[1]
        if xs[i]!=-1:
            nc[1-xs[i]]=nc[xs[i]]
        c=nc
    return min(*c)
    
for t in range(int(input())):
    x,y,xs=input().split()
    ans=go(int(x),int(y),[{'C':0, 'J':1, '?': -1}[x] for x in xs])
    print('Case #{}: {}'.format(t+1, ans))
