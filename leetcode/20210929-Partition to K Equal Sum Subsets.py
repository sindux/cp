# %%
#import plotly.express as px
import unittest
from typing import List, Dict, OrderedDict, Optional
import numpy as np
from functools import lru_cache,reduce
from queue import Queue, PriorityQueue
from collections import defaultdict, Counter, deque
import itertools as it
import math
import bisect
from numpy.lib.stride_tricks import as_strided
from timeit import timeit
from bisect import bisect_left, bisect_right
import sys
from dataclasses import dataclass,field
import heapq
import random
import array
import pandas as pd
import re
import string

from segmenttree import SegmentTree
from fenwicktree import FenwickTree
from treenode import TreeNode, fromlist, tolist, splevel, findnodeintree
from listnode import ListNode, tolistnode, fromlistnode
from listnoderandom import ListNodeRandom, tolistnoderandom, fromlistnoderandom
from narytree import fromlist as fromlistnarytree
from etc import logm, logmgen, as_set, UnionFind
from trie import Trie, trie_add, trie_in, trie_get, trie_gets#, trie_gets_bfs
from sortedcontainers import SortedList, SortedDict

#%%
class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        t = sum(nums)/k
        if int(t)!=t: return False

        nums.sort(reverse=True)
        def go(i, gsum):
            if i>=len(nums): 
                print(gsum,t)
                return True
            for gs in range(k):
                if gsum[gs]+nums[i]<=t:
                    gsum[gs]+=nums[i]
                    if go(i+1,gsum): return True
                    gsum[gs]-=nums[i]
            return False
        return go(0, [0]*k)


class Test(unittest.TestCase):
    def setUp(self):
        self.issimul='Solution' not in globals()

        self.doassert = True
        self.testcases = [
            ( [[730,580,401,659,5524,405,1601,3,383,4391,4485,1024,1175,1100,2299,3908],4],  True),
            # ( [[4,3,2,3,5,2,1], 4],  True),
            # ( [[1,2,3,4], 3],  False),
            # ( [[2]*16, 16],  True),
            # ( [[2]*8+[1]*8, 4],  True),
            ]

        # def verify(actual):
        #     x,y=actual
        #     self.assertTrue(x*x+y*y<=1)
        if self.issimul:
            self.testcases = [  
                ([["MapSum", "insert", "sum", "insert", "sum"],
                [[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]], 
                    [None, None, 3, None, 5]),
                ([["MapSum", "insert", "sum", "insert", "insert", "sum"],
                [[], ["apple",3], ["ap"], ["app",2], ["apple", 2], ["ap"]]], 
                    [None, None, 3, None, None, 4])
                    
                ]

    def runsimul(self, methods, args, expected):
        inst = None
        for m,a,e in zip(methods, args, expected):
            if not inst:
                print('******************')
                inst = globals()[m](*a)
                continue
            method = getattr(inst, m)
            print('===== BEGIN', [m,a,e])
            actual = method(*a)
            self.assertResult(e, actual, [m,a,e])
            
    def assertResult(self, expected, ans, msg):
        if isinstance(ans, ListNode):
            ans = fromlistnode(ans)
        elif isinstance(ans, TreeNode):
            ans = tolist(ans)
        elif isinstance(ans, ListNodeRandom):
            ans = fromlistnoderandom(ans)
        elif isinstance(ans, list) and ans and isinstance(ans[0], TreeNode):
            ans = [tolist(a) for a in ans]
        if isinstance(expected, set):
            # self.assertEqual(len(expected), len(ans), "expected:" + str(expected) + " ans: " + str(ans) + 
            #     "missing: " + str(expected-set(ans)) + "extra: " + str(set(ans)-expected))
            ans = set(ans)
        elif callable(expected):
            expected(ans)
            return
        print('----- END', msg, ans)
        if self.doassert:
            self.assertEqual(expected, ans)
        else:
            print('----- ASSERT SKIPPED')

    def test(self):
        for test in self.testcases:
            if self.issimul:
                self.runsimul(test[0][0], test[0][1], test[1])
                continue
            method = dir(Solution)[-1]  # last method
            self.sol = getattr(Solution(), method)
            # if isinstance(test[0], TreeNode):
            #     self.assertEqual(
            #         test[1], self.sol(test[0]))
            # elif isinstance(test[0], ListNode):
            #   self.assertListEqual(test[1], list(fromlistnode(self.sol(test[0]))))
            # elif isinstance(test[1], list):
            #     self.sol(test[0])
            #     self.assertListEqual(test[1], test[0])
            # else:     
            msgtest = test if len(str(test))<500 else "too long"
            print('===== BEGIN', msgtest)
            ans = self.sol(*test[0])
            self.assertResult(test[1], ans, msgtest)
            
unittest.main(argv=[''], exit=False)
# %%
16**16