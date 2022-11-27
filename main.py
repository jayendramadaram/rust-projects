f = int(input())
s = int(input())
t = int(input())
prev = f+s+t
count = 0
while True:
    j = input()
    if j.strip() == "":
        break
    f = s
    s = t
    t = int(j)
    summ = f+s+t
    if summ > prev:
        count += 1
    prev = summ
print(count)
