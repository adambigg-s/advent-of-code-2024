

import sys
import re


def main():
    file_path = "testing.txt" if len(sys.argv) < 2 else sys.argv[1]
    testing = len(sys.argv) < 2

    buffer = open(file_path, 'r').read()
    if testing:
        print(buffer)

    part_one = Solution.solve_one(buffer)
    part_two = Solution.solve_two(buffer)

    print(f"part one: {part_one}")
    print(f"part two: {part_two}")


class Solution:
    def solve_one(buffer) -> int:
        pattern = r"mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)"
        matches = re.findall(pattern, buffer)
        total = sum(int(x) * int(y) for x, y in matches)

        return total

    def solve_two(buffer) -> int:
        pattern = r"(do\(\)|don't\(\)|mul\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\))"
        matches = re.findall(pattern, buffer)

        enabled = True
        total = 0
        for match in matches:
            instruction = match[0]
            if instruction == "do()":
                enabled = True
            elif instruction == "don't()":
                enabled = False
            elif instruction.startswith("mul(") and enabled:
                x = int(match[1])
                y = int(match[2])
                
                total += x * y

        return total


if __name__ == "__main__":
    main()
