#%%
from treenode import splevel
from typing import List
from etc import logm
"""
class STNode:
    def __init__(self, l, r, val, lnode, rnode):
        self.l = l
        self.r = r
        self.val = val
        self.left = lnode
        self.right = rnode

class SegmentTree:
    def __init__(self, arr, fn = max):
        self.root = self.build(arr, 0, len(arr), fn)
        self.fn=fn
    def build(self, arr, l, r, fn):
        if l == r-1:
            return STNode(l, r, arr[l], None, None)
        mid = l + (r-l)//2
        lnode = self.build(arr, l, mid, fn)
        rnode = self.build(arr, mid, r, fn)
        val = fn(lnode.val, rnode.val)
        return STNode(l, r, val, lnode, rnode)

    def _update(self, i, v, node, merge, updfn):
        if node.l == i and node.r == i+1:
            #print('update', [node.l,node.r], node.val, v)
            node.val = updfn(node.val, v) if updfn else v
            return v
        mid = node.l + (node.r - node.l)//2
        if i < mid:
            updv = self._update(i, v, node.left, merge, updfn)
            otherv = self._q(mid, node.r, merge, node.right)
        else:
            updv = self._update(i, v, node.right, merge, updfn)
            otherv = self._q(node.l, mid, merge, node.left)
        node.val = merge(updv, otherv)
        #print('update', [node.l, node.r], v, otherv, node.val)
        return node.val

    def update(self, i, v, merge = max, updfn = None):
        self._update(i,v,self.root, merge, updfn)

    def _walk(self, node):
        if not node: return
        self._walk(node.left)
        self._walk(node.right)
        print([node.l, node.r], node.val)

    def print(self):
        self._walk(self.root)

    def _q(self, l, r, merge = max, node = None, level=0):
        assert node.l <= l and r <= node.r, "Out of boundary range query " + str([[l, r], [node.l, node.r]])
        if node.l == l and node.r == r:
            #print(splevel(level), [node.l, node.r], [l, r], node.val)
            return node.val

        mid = node.l + (node.r - node.l)//2
        ansl = None
        ansr = None
        if l < mid:
            ansl = self._q(l, min(mid, r), merge, node.left, level+1)
        if r > mid:
            ansr = self._q(max(mid, l), r, merge, node.right, level+1)
        ans =  merge(ansl, ansr) if ansl is not None and ansr is not None else ansl if ansl is not None else ansr
        #print(splevel(level), [node.l, node.r],  [l, r], ansl, ansr, ans)
        return ans

    def query(self, l, r, merge = max):
        return self._q(l, r, merge, self.root)

# st=SegmentTree([0,1,2,3,4,5])
# st.print()

# st.update(5,1)
# st.print()
# st.update(4,1)
# st.print()

#print(st.query(0,6))
# for i in range(6):
#     for j in range(i+1, 7):
#         print([i,j], st.query(i,j))
"""
class SegmentTree:
    def __init__(self, nums: List[int], fn=sum):
        self.N = len(nums)
        self.t = [0] * self.N*4
        self.fn = fn
        def do(i,l,r):
            if l==r-1:
                self.t[i] = nums[l]
                return self.t[i]
            mid = l + (r-l)//2
            ll = do(i*2+1, l, mid)
            rr = do(i*2+2, mid, r)
            self.t[i] = fn([ll, rr])
            return self.t[i]
        do(0, 0, self.N)
        # print(self.t)

    def update(self, index: int, val: int) -> None:
        # @logm
        def do(i, l, r, level=0):
            if l==r-1:
                self.t[i] = val
                return
            mid = l+(r-l)//2
            if index < mid:
                do(i*2+1, l, mid,level+1)
            else:
                do(i*2+2, mid, r,level+1)
            self.t[i] = self.fn(self.t[i*2+1], self.t[i*2+2])
        do(0, 0, self.N)

    def range(self, left: int, right: int, initial=0) -> int:
        """left & right inclusive"""
        # @logm
        def do(i, l, r, ql, qr, level=0):
            # print(l,r,ql,qr)
            if l==ql and r==qr:
                return self.t[i]
            ans = initial
            mid = l + (r-l)//2
            if ql < mid:
                ans = self.fn([ans, do(i*2+1, l, mid, max(l, ql), min(mid, qr), level+1)])
            if qr > mid:
                ans = self.fn([ans, do(i*2+2, mid, r, max(mid, ql), min(r, qr), level+1)])
            return ans
        return do(0, 0, self.N, left, right+1)
