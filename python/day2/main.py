from pathlib import Path
from argparse import ArgumentParser
from math import log10


def parse(file: Path):
    with file.open() as fp:
        for pair in fp.readline().split(","):
            parts = pair.split("-")
            yield int(parts[0]), int(parts[1])


def repints_in_range(L, R, only_k2=False):
    dL, dR = int(log10(L)) + 1, int(log10(R)) + 1
    seen = set() if not only_k2 else None
    
    for d in range(dL, dR + 1):
        for m in range(1, d):
            if d % m != 0:
                continue
            k = d // m
            if k < 2 or (only_k2 and k != 2):
                continue
            
            base = 10 ** m
            multiplier = (base ** k - 1) // (base - 1)
            start = 10 ** (m - 1)
            end = base
            
            for b in range(start, end):
                repint = b * multiplier
                if repint > R:
                    break
                if repint >= L:
                    if only_k2:
                        yield repint
                    elif repint not in seen:
                        seen.add(repint)
                        yield repint


def solve1(data: list[tuple[int, int]]):
    acc = 0
    for rg in data:
        acc += sum(repints_in_range(*rg, only_k2=True))
    return acc


def solve2(data: list[int]):
    acc = 0
    for rg in data:
        acc += sum(repints_in_range(*rg))
    return acc
    

parser = ArgumentParser()
parser.add_argument("input", type=Path)
args = parser.parse_args()

nums = list(parse(args.input))
print(f"Day2 part1: {solve1(nums)}")
print(f"Day2 part2: {solve2(nums)}")

