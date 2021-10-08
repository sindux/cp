import numpy as np


def do(k, tm):
    tm.sort(axis=0)
    res=0
    for i in tm:
        res+=1
        
t=int(input())
for i in range(t):
    n,k=np.fromstring(input(), sep=' ', dtype=int)
    tm=np.zeros((n,2),dtype=int)
    for j in range(n):
        tm[j]=np.fromstring(input(), sep=' ', dtype=int)
    print('Case {}: {}'.format(i+1,tm))