from pathlib import Path
from argparse import ArgumentParser


def parse(file: Path):
    with file.open() as fp:
        while line := fp.readline():
            yield int(line.strip().replace("R", "").replace("L", "-"))


def solve1(data: list[int]):
    val = 50
    cnt = 0
    for n in data:
        val = (val + n) % 100
        if val == 0:
            cnt += 1
    return cnt


def solve2(data: list[int]):
    val = 50
    cnt = 0
    for n in data:
        if n > 0:
            cnt += (val + n) // 100 - val // 100
        elif n < 0:
            cnt += (val - 1) // 100 - (val + n - 1) // 100
        val = (val + n) % 100
    return cnt
    

parser = ArgumentParser()
parser.add_argument("input", type=Path)
args = parser.parse_args()

nums = list(parse(args.input))
print(solve1(nums))
print(solve2(nums))

