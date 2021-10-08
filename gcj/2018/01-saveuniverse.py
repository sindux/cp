# Saving the Universe again
def parse(prog):
    ans=[0]
    for s in prog:
        if s == 'C':
            ans.append(0)
        else:
            ans[-1]=ans[-1]+1
    return ans

def eval(d, prog, idx, ans):
    if (idx < 0):
        return "IMPOSSIBLE" if d < 0 else ans

    power = (1 << idx)
    maxshoot = d // power
    for take in range(min(prog[idx], maxshoot), -1, -1):
        newd = d-take*power
        assert newd >= 0
        thisans=ans
        if take < prog[idx]:
            thisans += prog[idx] - take
            if idx > 0:
                prog[idx-1] += prog[idx] - take
            else:
                newd=-1
        res = eval(newd, prog, idx-1, thisans)

        if take < prog[idx]:
            if idx > 0:
                prog[idx-1] -= prog[idx]-take
        if res != "IMPOSSIBLE":
            return res
    return "IMPOSSIBLE"
        

def doit(d, prog):
    prog=parse(prog)
    return eval(d,prog,len(prog)-1, 0)
    
for i in range(int(input())):
    d,prog=input().split(' ')
    ans=doit(int(d), prog)
    print("Case #" + str(i+1) + ": " + str(ans))