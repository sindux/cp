#%%
class FenwickTree:
    def __init__(self, size):
        self.arr = [0]*size
    def add(self, at, by):
        while at < len(self.arr):
            self.arr[at] += by
            at |= (at+1)
    def query(self, at):
        res = 0
        while at>=0:
            res += self.arr[at]
            at = (at & (at+1))-1
        return res

# a=FenwickTree(10)
# for i in range(10):
#     a.add(i,i)
#     print(a.query(i))

# class FenwickTree:
#     def __init__(self, size):
#         self.A = [0] * size
#     def sum(self, i):
#         ans=0
#         while i>0:
#             ans += self.A[i-1]
#             i = i & (i-1)
#         return ans
#     def range_sum(self, i, j):
#         ans=0
#         while j>i:
#             ans += self.A[j-1]
#             j=j&(j-1)
#         while i>j:
#             ans-=self.A[i-1]
#             i=i&(i-1)
#         return ans

#     def add(self, i, delta):
#         while i<len(self.A):
#             self.A[i]+=delta
#             i = i | (i+1)