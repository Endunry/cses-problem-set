s = input()
diff = set(s)
m = 0
for d in diff:
    c = s.count(d)
    m = max(m,c)
print(m)