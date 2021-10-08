def do(nums, idx, res):
    if idx >= len(nums):
        return True
    t=nums[idx]
    ok = False
    for i in range(t, -1, -1):
        if i == 4: continue
        j = t - i
        if j == 4: continue
        res.append(i)
        ok = do(nums, idx+1, res)
        if not ok:
            res.pop()
        break
    if ok:
        return True
    if idx < len(nums)-1:
        for i in range(t+1, 10):
            if i == 4: continue
            j = 10+t-i
            if j == 4: continue
            nums[idx+1]-=1
            res.append(i)
            ok = do(nums, idx+1, res)
            nums[idx+1]+=1
            if not ok:
                res.pop()
            break
            
    return ok

from functools import reduce
def main():
    for i in range(int(input())):
        n = int(input())
        nums = list(map(int, list(str(n))[-1::-1]))
        res=[]
        do(nums, 0, res)
        #print(res)
        a = reduce(lambda acc,i: acc*10+i, res[-1::-1], 0)
        #print(a,n,n-a)
        print('Case #' + str(i+1) + ': ' + str(a) +' '+ str(n-a))

main()