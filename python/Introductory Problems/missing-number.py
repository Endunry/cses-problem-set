arr = set(range(1,int(input())+1))
s = set((map(int,input().split(' '))))
print((arr-s).pop())