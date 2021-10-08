import inspect

def _get_level_pos(params):
    lev = [i for i,v in enumerate(params) if v == 'level']
    return lev[0] if lev else -1

def _get_level_space(level_pos, args, kwargs):
    if level_pos>=0:
        if len(args) >= level_pos+1:
            level = args[level_pos]
        else:
            level = kwargs.get('level', 0)
        return str(level) + '. ' + '    ' * level
    return ''

def _map_params(params, args):
    for p,a in zip(params, args):
        if p in ['self', 'level']: continue
        if p.endswith('_mask'): yield bin(a)
        else: yield a

def logm(func):
    params = inspect.signature(func).parameters
    level_pos = _get_level_pos(params)
    def _wrap(*args, **kwargs):
        # print(_get_level_space(level_pos, args, kwargs), list(_map_params(params, args)), 
        #     kwargs if kwargs else '', ' -> ', None, sep='')
        ans = func(*args, **kwargs)
        print(_get_level_space(level_pos, args, kwargs), list(_map_params(params, args)), 
            kwargs if kwargs else '', ' => ', ans, sep='')
        return ans
    return _wrap

def logmgen(func):
    params = inspect.signature(func).parameters
    level_pos = _get_level_pos(params)
    def _wrap(*args, **kwargs):
        ans = func(*args, **kwargs)
        if ans is None:   # how to make this work?
            print(_get_level_space(level_pos, args, kwargs), list(_map_params(params, args)),
                kwargs if kwargs else '', ' => ', ans, sep='')
        else:
            for a in ans:
                print(_get_level_space(level_pos, args, kwargs), list(_map_params(params, args)),
                    kwargs if kwargs else '', ' => ', a, sep='')
                yield a
    return _wrap

def as_set(l):
    return set(tuple(i) if isinstance(i, list) else i for i in l)


class UnionFind:
    def __init__(self, n):
        self.p = list(range(n))
        self.s = [1] * n
    
    def find(self, i):
        ii = i
        while self.p[i] != i:
            i = self.p[i]
        while self.p[ii] != i:
            self.p[ii],ii = i,self.p[ii]
        return i, self.s[i]
    
    def union(self, i, j):
        ii,si = self.find(i)
        jj,sj = self.find(j)
        if ii != jj:
            # self.s[ii] = self.s[jj] = si+sj
            # while self.p[j]!=j:
            #     self.p[j], j = ii,self.p[j]
            # self.p[j]=ii
            if self.s[ii] < self.s[jj]:
                self.p[ii] = jj
            else:
                self.p[jj] = ii
            self.s[ii] = self.s[jj] = si+sj



# uf=UnionFind(10)
# uf.union(0,1)
# uf.union(2,3)
# uf.union(4,5)
# #uf.union(3,4)
# uf.union(6,7)
# uf.union(9,8)
# uf.union(0,2)
# [uf.find(i) for i in range(10)]
# #print(uf.p)



    