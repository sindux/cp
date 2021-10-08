from collections import deque
from typing import List
class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        def doit():
            q = deque([],k)
            q.append(0)
            if k==1:
                yield nums[0]
            for idx,n in enumerate(nums[1:]):
                if n == 9774:
                    print(idx,[(ii,nums[ii]) for ii in q])
                idx+=1
                minidx=idx-k+1
                while len(q)>0 and q[0]<minidx:
                    q.popleft()
                if len(q)>0 and n >= nums[q[0]]:
                    q.popleft()
                    q.appendleft(idx)
                else:
                    while len(q)>0 and nums[q[-1]]<n:
                        q.pop()
                    q.append(idx)
                #print(n,q)
                if idx>=k-1:
                    yield nums[q[0]]
        return list(doit())


import json
for i in range(int(input())):
    nums=json.loads(input())
    k=int(input())
    print(Solution().maxSlidingWindow(nums,k))
