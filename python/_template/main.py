from pathlib import Path
from argparse import ArgumentParser


def parse(file: Path):
    with file.open() as fp:
        pass


def solve1(data: list[int]):
    pass


def solve2(data: list[int]):
    pass
    

parser = ArgumentParser()
parser.add_argument("input", type=Path)
args = parser.parse_args()

nums = list(parse(args.input))
print(f"Day@DAY@ part1: {solve1(nums)}")
print(f"Day@DAY@ part2: {solve2(nums)}")

