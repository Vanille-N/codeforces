def cost(a):
    s = 0
    for x in a:
        if x < 0:
            m = min(s, abs(x))
            s -= m
        else:
            s += x
    return s

t = int(input())
for _ in range(t):
    n = int(input())
    a = list(map(int, input().split(" ")))
    print(cost(a))
