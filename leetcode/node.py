from queue import Queue

class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next
    def print(self,parent=-1,level=0,lr=0):
        yield level, parent, self.val, self.next.val if self.next else None
        if self.left:
            yield from self.left.print(self.val,level+1,0)
        if self.right:
            yield from self.right.print(self.val,level+1,0)
    def p(self):
        return sorted(self.print())

def tolist(tree: Node):
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
    return list(map(lambda x:x[0], filter(lambda x:x[1]!=lastlevel, xs)))

def tolistnext(tree: Node):
    xs=[]
    q = Queue()
    q.put((tree,0))
    while not q.empty():
        node,level = q.get()
        xs.append((node.val, (node.right.val if node.right else None),level) if node else (None,level))
        if node:
            q.put((node.left,level+1))
            q.put((node.right,level+1))
    # return xs
    lastlevel = xs[-1][1]
    return list(map(lambda x:(x[0],x[1]), filter(lambda x:x[-1]!=lastlevel, xs)))


def fromlist(xs) -> Node:
    q = Queue()
    head = Node(xs[0])
    idx = 1
    q.put((0, head))
    q.put((1, head))
    while idx < len(xs):
        lr, node = q.get()
        if xs[idx] is not None:
            newnode = Node(xs[idx])
            if lr==0    :
                node.left = newnode
            else:
                node.right = newnode
            q.put((0,newnode))
            q.put((1,newnode))
        idx += 1
    return head


def print_mem():
    import os,psutil
    process=psutil.Process(os.getpid())
    print(process.memory_info())