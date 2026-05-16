# borders

Add white coloured borders to images.
Portrait images get top/bottom padding; landscape images get side padding both are then expanded to a target aspect ratio.

## Build & install

```sh
make link   # builds release binary and symlinks to $(HOME)/.local/bin/borders
make build  # output at target/release/borders
make clean  # cargo clean
```

## Usage

```sh
Add borders to images.

Usage: borders [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Input image files to process

Options:
      --top <TOP>                  Top border as a fraction of image height (portrait) [default: 0.01]
      --bottom <BOTTOM>            Bottom border as a fraction of image height (portrait) [default: 0.02]
      --side <SIDE>                Side border as a fraction of image width (landscape) [default: 0.01]
      --top-wide <TOP_WIDE>        Fraction of vertical border placed on top (landscape); must sum to 1.0 with --bottom-wide [default: 0.45]
      --bottom-wide <BOTTOM_WIDE>  Fraction of vertical border placed on bottom (landscape); must sum to 1.0 with --top-wide [default: 0.55]
      --ratio <RATIO>              Target aspect ratio (width/height) for the output image [default: 0.8]
      --output-dir <OUTPUT_DIR>    Directory to write output images into [default: with_borders]
      --quality <QUALITY>          JPEG output quality (1–100); ignored for non-JPEG formats [default: 100]
      --color <COLOR>              Border colour as a hex string (e.g. ffffff) [default: ffffff]
  -h, --help                       Print help
```

### Examples

```sh
borders photo1.jpg photo2.jpg
borders --color 000000 --quality 85 --output-dir processed *.jpg
```
