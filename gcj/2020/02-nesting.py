def do(s):
    ans=''
    l=0
    for c in map(int,s):
        if c>l:
            for _ in range(c-l):
                l+=1
                ans+='('
        elif c<l:
            for _ in range(l-c):
                l-=1
                ans+=')'
        ans+=str(c)
    for _ in range(l):
        ans+=')'
    return ans
t=int(input())
for i in range(t):
    s=input()
    ans=do(s)
    print('Case #{0}: {1}'.format(i+1,ans))