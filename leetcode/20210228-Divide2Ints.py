#%%
from collections import deque
from typing import List
from queue import Queue
import string
from bisect import bisect_left, bisect_right
import numpy as np

class Solution:
    def divide(self, dividend: int, divisor: int) -> int:
        ans=0
        sign=1 if dividend>=0 and divisor>=0 or dividend<0 and divisor<0 else -1
        dividend=abs(dividend)
        divisor=abs(divisor)
        while dividend>=divisor:
            div=divisor
            tans=1
            while dividend>=div:
                tans<<=1
                div<<=1
            print(dividend,div,tans)
            ans+=tans>>1
            dividend-=(div>>1)
            print(dividend,ans)
        return min(ans if sign==1 else -ans, 2**31-1)
        

import unittest  

class TestCase(unittest.TestCase):  
    def testAdd(self):  # test method names begin with 'test'  
        testcases = [ ([10,3], 3),   # 3*2^1 + 3*2^0 = 3*3
                      ([15,3], 5),   # 3*4 + 3
                      ([7,-3], -2),  # -3 * 2^1
                      ([0,1], 0),
                      ([1,1], 1),
                      ([3*2**10+3*2**5+3*1,3],2**10+2**5+1),
                      #([2147483648,1],2147483648),
                      ([-2147483648,-1],2147483647),
                      ]

        for testarg,testres in testcases:
            print(testarg,testres)
            ans = Solution().divide(*testarg)
            self.assertEqual(testres, ans)

unittest.main(argv=[''],exit=False)

#%%
-2147483648/-1
min(2**31-1,2147483648)