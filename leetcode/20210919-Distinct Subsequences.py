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
    def numDistinct(self, s: str, t: str) -> int:
        @lru_cache
        def go(i,j,level=0):
            if i == 0: 
                return 0 if j>0 else 1
            if j == 0: 
                return 1
            w1 = go(i-1, j-1,level+1) if s[i-1]==t[j-1] else 0 # keep
            w2 = go(i-1,j,level+1) # delete
            return w1 + w2
        return go(len(s), len(t))

        
class Test(unittest.TestCase):
    def setUp(self):
        self.issimul='Solution' not in globals()

        self.doassert = True
        self.testcases = [
            ( ["a", ""],  1),
            ( ["", "a"],  0),
            ( ["aa", "a"],  2),
            ( ["rabbbit", "rabbit"],  3),
            ( ["babgbag", "bag"],  5),
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
for i in range(10):
    for j in range(10):
        print(f'{i} * {j} = {i*j}')