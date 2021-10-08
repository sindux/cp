def isodd(i):
    return i%2 == 1

def doge(s, idx=0, first=True):
    print(s,idx,first)
    if idx>=len(s):
        return 1
    d=int(s[idx])
    mm=11 if isodd(idx+1) else 10
    if first:
        fd = 1 if isodd(d) else 0
        ans=fd*doge(s,idx+1,True) + (mm-d-fd)//2 * doge(s,idx+1,False)
        return ans
    return 5*doge(s,idx+1,first)

print(doge('102'))
# #1:
# 1
# 3
# 5
# 7
# 9

# 0
# 2
# 4
# 6
# 8

def isb(s):
    for i,d in enumerate(s):
        d=int(d)
        if (isodd(i+1) and not isodd(d)) or (not isodd(i+1) and isodd(d)):
            return False
    return True

def dole(s):
    yy = 5**len(s)-doge(s)
    baseb = not isodd(len(s))
    return yy + (1 if isb(s) and not baseb else 0) + (1 if baseb else 0)

def do(l,r):
    ln = len(str(l))
    rn = len(str(r))
    if ln == rn:
        return 5**ln - dole(str(l)) - doge(str(r)) + (1 if isb(str(l)) else 0) + (1 if isb(str(r)) else 0)
    ans = 0
    for ll in range(ln, rn+1):
        if ll==ln:
            ans += doge(str(l))
        elif ll==rn:
            ans += dole(str(r))
        else:
            ans += 5**ll
        #print(ll,ans)
    return ans

print(do(5,15))
# for t in range(int(input())):
#     l,r=list(map(int, input().split()))
#     res=do(l,r)
#     print('Case #{}: {}'.format(t+1, res))
