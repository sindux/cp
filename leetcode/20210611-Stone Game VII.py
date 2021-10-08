#%%
from collections import deque
from typing import List
from queue import Queue
import string
from bisect import bisect_left, bisect_right
import numpy as np

class Solution:
    def stoneGameVII(self, stones: List[int]) -> int:
        def go(l,r,sumr):
            if sumr==0:
                return 0
            l = sumr-stones[l] + go(l+1, r, sumr-stones[l])
            r = sumr-stones[r] + go(l, r-1, sumr-stones[r])
            .
            
# 1,2
        
        
# 1,2,3
A B A   A     B   d
1 2 3    5    3   2  --- 
1 3 2    5    2   3
3 1 2    3    2   1
3 2 1    3    1   2


import unittest  

class TestCase(unittest.TestCase):  
    def testAdd(self):  # test method names begin with 'test'  
        testcases = [ ([ [5,3,1,4,2]], 6),
           ([ [7,90,5,1,100,10,10,2]], 122)
        ]

        for testarg,testres in testcases:
            print(testarg,testres)
            ans = Solution().divide(*testarg)
            self.assertEqual(testres, ans)

unittest.main(argv=[''],exit=False)

#%%
-2147483648/-1
min(2**31-1,2147483648)