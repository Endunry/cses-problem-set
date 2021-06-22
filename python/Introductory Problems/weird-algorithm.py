from sys import argv
n = int(argv[1])
while(n!=1):
    print(n)
    n = n*3+1 if n%2 else n>>1
