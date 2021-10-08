#%%
from queue import Queue
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
    # def __repr__(self):
    #     return f"{self.val} L:{self.left}-{self.val} R:{self.right}-{self.val}"
    def print(self,parent=-1,level=0,lr=0):
        yield level, parent, self.val
        if self.left:
            yield from self.left.print(self.val,level+1,0)
        if self.right:
            yield from self.right.print(self.val,level+1,0)
    def p(self):
        return sorted(self.print())
    def __repr__(self):
        return f'(TN: {self.val} {"L" if self.left else "-"} {"R" if self.right else "-"})'
    
def splevel(level):
    return str(level) + '. ' + ''.join(['  '] * level)

def tolist(tree: TreeNode):
    xs=[]
    q = Queue()
    q.put((tree,0))
    while not q.empty():
        node,level = q.get()
        xs.append((node.val,level) if node else (None,level))
        if node:
            q.put((node.left,level+1))
            q.put((node.right,level+1))
    # return xs
    lastlevel = xs[-1][1]
    # drop last level
    xs = list(map(lambda x:x[0], filter(lambda x:x[1]!=lastlevel, xs)))
    # drop last None
    while xs[-1] is None:
        xs.pop()
    return xs

def fromlist(xs) -> TreeNode:
    if not xs: return None
    q = Queue()
    head = TreeNode(xs[0], None, None)
    idx = 1
    q.put((0, head))
    q.put((1, head))
    while idx < len(xs):
        lr, node = q.get()
        if xs[idx] is not None:
            newnode = TreeNode(xs[idx], None, None)
            if lr==0:
                node.left = newnode
            else:
                node.right = newnode
            q.put((0,newnode))
            q.put((1,newnode))
        idx += 1
    return head
    # if idx >= len(xs) or xs[idx] is None: return None
    # left = fromlist(xs, idx*2+1)
    # right = fromlist(xs, idx*2+2)
    # return TreeNode(xs[idx], left, right)

#tolist(fromlist([5,3,6,2,4,None,8,1,None,None,None,7,9]))
#tolist(fromlist([1,None,2,None,3,None,4,None,5,None,6,None,7,None,8,None,9]))
#fromlist([1,None,2,None,3,None,4,None,5,None,6,None,7,None,8,None,9]).p()

def findnodeintree(root: TreeNode, val):
    if not root: return
    if root.val == val:
        return root
    ans = findnodeintree(root.left, val)
    if not ans:
        return findnodeintree(root.right, val)
    return ans
    
