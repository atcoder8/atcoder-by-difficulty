import os
import pathlib
from typing import Iterable


MIN_PROBLEM_NUMBER = 284
MAX_PROBLEM_NUMBER = 283

BIN_FILE_PATHNAME = "./src/bin/abc{:03d}-a.rs"
CARGO_CONFIG_FILE_PATHNAME = "./Cargo.toml"

TEMPLATE = """\
fn main() {
    
}
"""

DEFINE_BINARY_TARGET_FORMAT = """
[[bin]]
name = "abc{0:03d}-a"
path = "src/bin/abc{0:03d}-a.rs"
"""


def create_files(src_file_path_list: Iterable[os.PathLike], template: str):
    for src_file_path in src_file_path_list:
        src_file_path = pathlib.Path(src_file_path)
        src_file_path.parent.mkdir(parents=True, exist_ok=True)
        src_file_path.write_text(template, encoding="utf-8")


if __name__ == "__main__":
    cargo_config_path = pathlib.Path(CARGO_CONFIG_FILE_PATHNAME)

    for problem_number in range(MIN_PROBLEM_NUMBER, MAX_PROBLEM_NUMBER + 1):
        with open(cargo_config_path, mode="a", encoding="utf-8") as cargo_config_file:
            cargo_config_file.write(DEFINE_BINARY_TARGET_FORMAT.format(problem_number))

        src_file_path = pathlib.Path(BIN_FILE_PATHNAME.format(problem_number))
        src_file_path.parent.mkdir(parents=True, exist_ok=True)
        src_file_path.write_text(TEMPLATE, encoding="utf-8")
