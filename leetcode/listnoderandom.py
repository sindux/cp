#%%
class ListNodeRandom:
    def __init__(self, val=0, next=None, random=None):
        self.val = val
        self.next = next
        self.random = random

def tolistnoderandom(xs):
    head = None
    prev = None
    idxnode = {}
    for i,x in enumerate(xs):
        cur = ListNodeRandom(x[0])
        idxnode[i]=cur
        if not head: head = cur
        if prev: prev.next = cur
        prev = cur
    nx = head

    for x in xs:
        if x[1] is not None:
            tgt = idxnode[x[1]]
            nx.random = tgt
        nx = nx.next
    return head

def fromlistnoderandom(x: ListNodeRandom):
    def f(nodeidxmap):
        nonlocal x
        idx = 0
        while x:
            yield [x.val, nodeidxmap[x.random] if x.random is not None else None]
            x = x.next
            idx+=1
    c = x
    nodeidxmap = {}
    i = 0
    while c:
        nodeidxmap[c]=i
        i+=1
        c=c.next
    return list(f(nodeidxmap))


#list(fromlistnoderandom(tolistnoderandom([1,2,3])))
#list(fromlistnode(tolistnode([1])))
#fromlistnoderandom(tolistnoderandom([[1,None],[2,0]]))
#fromlistnoderandom(tolistnoderandom([[1,1],[2,0]]))
#tolistnoderandom([[1,None],[2,0]])
#tolistnoderandom([])

