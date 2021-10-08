#%%
from queue import Queue
class NaryNode:
    def __init__(self, val=0, children=None):
        self.val = val
        self.children = children
    
# def tolist(tree: NaryNode):
#     xs=[]
#     q = Queue()
#     q.put((tree,0))
#     while not q.empty():
#         node,level = q.get()
#         xs.append((node.val,level) if node else (None,level))
#         if node:
#             q.put((node.left,level+1))
#             q.put((node.right,level+1))
#     # return xs
#     lastlevel = xs[-1][1]
#     # drop last level
#     xs = list(map(lambda x:x[0], filter(lambda x:x[1]!=lastlevel, xs)))
#     # drop last None
#     while xs[-1] is None:
#         xs.pop()
#     return xs

def fromlist(xs) -> NaryNode:
    if len(xs)==0:
        return None
    q = Queue()
    head = NaryNode(xs[0], None)
    assert xs[1] is None
    idx = 2
    q.put(head)
    while idx < len(xs):
        node = q.get()
        while idx < len(xs) and xs[idx] is not None:
            newchild = NaryNode(xs[idx], None)
            if node.children is None:
                node.children = []
            node.children.append(newchild)
            q.put(newchild)
            idx+=1
        idx += 1
    return head

#tolist(fromlist([5,3,6,2,4,None,8,1,None,None,None,7,9]))
#tolist(fromlist([1,None,2,None,3,None,4,None,5,None,6,None,7,None,8,None,9]))
fromlist([1,None,3,2,4,None,5,6]).children[0].children[0].children

