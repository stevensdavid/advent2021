from sys import stdin

print(
    sum(
        1
        for l in stdin.readlines()
        for s in l.split(" | ")[1].split(" ")
        if len(s.strip()) in (2, 3, 4, 7)
    )
)
