from pathlib import Path
from argparse import ArgumentParser
from functools import reduce


def parse(file: Path):
    with file.open() as fp:
        for line in fp:
            yield [int(c) for c in line.strip()]


def pick_max(line, dl):
    def step(state, i):
        si, num = state
        ei = len(line) - (dl - i - 1)
        f = max(line[si:ei])
        return (line.index(f, si) + 1, num * 10 + f)
    return reduce(step, range(dl), (0, 0))[1]


def solve1(data, dl=2):
    return sum(pick_max(line, 2) for line in data)


def solve2(data):
    return sum(pick_max(line, 12) for line in data)
    

parser = ArgumentParser()
parser.add_argument("input", type=Path)
args = parser.parse_args()

nums = list(parse(args.input))
print(f"Day3 part1: {solve1(nums)}")
print(f"Day3 part2: {solve2(nums)}")

