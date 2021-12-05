from sys import stdin
from math import ceil

lines = stdin.readlines()
n_bits = len(lines[0]) - 1 # Assumes same number on each line
freqs = [0] * n_bits
for line in lines:
    freqs = map(lambda x: x[0] + int(x[1]), zip(freqs, filter(str.isdigit, line)))
bits = [int(x >= ceil(len(lines) / 2)) for x in freqs]
print(bits)
gamma = 0
for idx, bit in enumerate(reversed(bits)):
    gamma += bit << idx
epsilon = ~gamma & 0x1F
print(f"Gamma: {gamma} Epsilon: {epsilon} Product: {gamma * epsilon}")
