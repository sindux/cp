from collections import defaultdict, deque

def Trie():
    return defaultdict(Trie)

def trie_add(trie, word, terminator='  ', payload = None):
    trie = trie_get(trie, word, append=True)
    if terminator in trie:
        trie[terminator].append(payload)
    else:
        trie[terminator] = [payload]

def trie_in(trie, word, terminator='  '):
    trie = trie_get(trie, word)
    return terminator in trie

def trie_get(trie, prefix, append=False):
    for c in prefix:
        if append:
            trie = trie[c]
        else:
            trie = trie.get(c)
            if trie is None:
                break
    return trie


def trie_gets(trie, prefix='', sort=False, terminator='  '):
    for c in prefix:
        trie = trie.get(c)
        if trie is None:
            return None
    items = trie.items()
    if sort:
        items = sorted(items)
    for k,v in items:
        if k == terminator:
            yield prefix, v
        else:
            for sub, vv in trie_gets(v, '', sort, terminator):
                yield prefix + k + sub, vv

        
# def trie_gets_bfs(trie, prefix='', terminator='  '):
#     for c in prefix:
#         trie = trie.get(c)
#         if not trie:
#             return
#     q = deque(sorted((prefix+k, v if k!=terminator else terminator) for k,v in trie.items()))
#     print(q)
#     while q:
#         k,v = q.popleft()
#         if v == terminator:
#             yield k
#         else:
#             print(k,v)
#             for sub,vv in sorted(v.items()):
#                 q.append((k+sub, vv if sub!=terminator else terminator))

        