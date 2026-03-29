import argparse
import asyncio
from pathlib import Path
from shutil import which
import subprocess
from sys import exit


class Args(argparse.Namespace):
    files: list[str] = []
    top: float = 0.01
    bottom: float = 0.02
    side: float = 0.01
    top_wide: float = 0.45
    bottom_wide: float = 0.55
    ratio: float = 4 / 5
    output_dir: str = "./resized"


def main(args: Args):
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    tasks = [
        resize(
            file,
            output_dir,
            args.top,
            args.bottom,
            args.side,
            args.top_wide,
            args.bottom_wide,
            args.ratio,
        )
        for file in args.files
    ]
    loop.run_until_complete(asyncio.gather(*tasks))


def run_async(f):
    async def wrapped(*args, **kwargs):
        return await asyncio.to_thread(f, *args, **kwargs)

    return wrapped  # pyright: ignore[reportUnknownVariableType]


@run_async
def resize(
    file: str,
    output_dir: Path,
    top: float,
    bottom: float,
    side: float,
    top_wide: float,
    bottom_wide: float,
    ratio: float,
):
    input_file = Path(file)
    tmp_file = Path("/tmp") / input_file.name
    output_file = output_dir / input_file.name

    # Get image dimensions
    width = get_width(str(input_file))
    height = get_height(str(input_file))

    if width >= height:
        # Landscape
        border = int(width * side)
        add_side_border(str(input_file), str(tmp_file), border)
        new_width = get_width(str(tmp_file))
        new_height = int(new_width * (1 / ratio))
        total_border = new_height - height

        border_top = int(total_border * top_wide)
        border_bottom = int(total_border * bottom_wide)
        add_top_bottom_border(
            str(tmp_file), str(output_file), border_top, border_bottom
        )
    else:
        # Portrait
        border_top = int(height * top)
        border_bottom = int(height * bottom)
        add_top_bottom_border(str(input_file), str(tmp_file), border_top, border_bottom)

        new_height = get_height(str(tmp_file))
        new_width = int(new_height * ratio)
        total_border = new_width - width

        side_border = int(total_border / 2)
        add_side_border(str(tmp_file), str(output_file), side_border)


def get_width(img_file: str) -> int:
    return int(
        subprocess.check_output(["magick", "identify", "-format", "%w", img_file])
    )


def get_height(img_file: str) -> int:
    return int(
        subprocess.check_output(["magick", "identify", "-format", "%h", img_file])
    )


def add_side_border(input_file: str, output_file: str, border: int):
    _ = subprocess.run(
        [
            "magick",
            input_file,
            "-bordercolor",
            "white",
            "-border",
            f"{border}x0",
            output_file,
        ]
    )


def add_top_bottom_border(
    input_file: str, output_file: str, border_top: int, border_bottom: int
):
    _ = subprocess.run(
        [
            "magick",
            input_file,
            "-background",
            "white",
            "-gravity",
            "north",
            "-splice",
            f"0x{border_top}",
            "-gravity",
            "south",
            "-splice",
            f"0x{border_bottom}",
            output_file,
        ]
    )


def parse_args() -> Args:
    parser = argparse.ArgumentParser()
    parser.add_argument("files", type=str, nargs="+")  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--top", type=float, default=Args.top)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--bottom", type=float, default=Args.bottom)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--side", type=float, default=Args.side)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--top-wide", type=float, default=Args.top_wide)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--bottom-wide", type=float, default=Args.bottom_wide)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--ratio", type=float, default=Args.ratio)  # pyright: ignore[reportUnusedCallResult]
    parser.add_argument("--output_dir", type=str, default=Args.output_dir)  # pyright: ignore[reportUnusedCallResult]

    args = parser.parse_args(namespace=Args())
    assert (args.top_wide + args.bottom_wide) == 1
    return args


if __name__ == "__main__":
    args = parse_args()
    if which("magick"):
        main(args)
    else:
        exit("ImageMagick is not installed")
