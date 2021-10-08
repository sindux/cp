#%%
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    def __str__(self):
        return str(self.val)

def tolistnode(xs):
    head = None
    prev = None
    for x in xs:
        cur = ListNode(x)
        if not head: head = cur
        if prev: prev.next = cur
        prev = cur
    return head

def fromlistnode(x: ListNode):
    def f():
        nonlocal x
        while x:
            yield x.val
            x = x.next
    return list(f())


#list(fromlistnode(tolistnode([1,2,3])))
#list(fromlistnode(tolistnode([1])))
#tolistnode([])
