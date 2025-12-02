#!/usr/bin/env python3
"""Create a new Advent of Code day from templates."""

import argparse
import shutil
from pathlib import Path

# Root directory of the project
ROOT = Path(__file__).parent

# Languages to set up (each must have a _template folder)
LANGUAGES = ["rust", "python"]

# Extra variables to replace in templates (in addition to @DAY@)
EXTRA_VARIABLES = {
    # "@YEAR@": "2025",
}


def replace_variables(content: str, day: int) -> str:
    """Replace @VARIABLE@ placeholders in content."""
    variables = {"@DAY@": str(day), **EXTRA_VARIABLES}
    for var, value in variables.items():
        content = content.replace(var, value)
    return content


def setup_day(day: int):
    """Set up a new day for all languages."""
    day_folder = f"day{day}"

    for lang in LANGUAGES:
        template_dir = ROOT / lang / "_template"
        target_dir = ROOT / lang / day_folder

        if not template_dir.exists():
            print(f"Warning: Template not found for {lang}, skipping")
            continue

        if target_dir.exists():
            print(f"Warning: {target_dir} already exists, skipping")
            continue

        # Copy template directory structure
        shutil.copytree(
            template_dir,
            target_dir,
            ignore=shutil.ignore_patterns("target", "Cargo.lock", "__pycache__", "*.pyc"),
        )

        # Process all files and replace variables
        for file_path in target_dir.rglob("*"):
            if file_path.is_file():
                try:
                    content = file_path.read_text()
                    new_content = replace_variables(content, day)
                    if content != new_content:
                        file_path.write_text(new_content)
                        print(f"  Updated: {file_path.relative_to(ROOT)}")
                except UnicodeDecodeError:
                    # Skip binary files
                    pass

        print(f"Created {target_dir.relative_to(ROOT)}")


def main():
    parser = argparse.ArgumentParser(description="Create a new Advent of Code day")
    parser.add_argument("day", type=int, help="Day number to create")
    args = parser.parse_args()

    setup_day(args.day)


if __name__ == "__main__":
    main()
