import numpy as np
def go(xs):
    ans=0
    for i in range(len(xs)-1):
        j=np.argmin(xs[i:])+i
        ans+=j-i+1
        xs[i:j+1]=np.flip(xs[i:j+1])
    return ans
    
for t in range(int(input())):
    input()
    xs = np.array([int(i) for i in input().split()])
    print(xs)
    print('Case #{}: {}'.format(t+1, go(xs)))