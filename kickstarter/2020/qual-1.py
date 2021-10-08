import numpy as np

def do(x,a):
    b=(a-1)//x
    return np.argsort(b, kind='stable')+1

t = int(input())
for i in range(t):
    _,x=np.fromstring(input(), dtype=int, sep= ' ')
    a=np.fromstring(input(), dtype=int, sep=' ')
    ans=do(x,a)
    print('Case #{}: {}'.format(i+1, " ".join(map(str,ans))))