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
  [FILES]...

Options:
      --default-horizontal <DEFAULT_HORIZONTAL>
          Left/right borders as a fraction of image height [default: 0.02]
      --default-vertical <DEFAULT_VERTICAL>
          Top/bottom borders as a fraction of image width [default: 0.03]
      --fraction-top <FRACTION_TOP>
          Fraction of vertical border placed on top; must sum to 1.0 with --fraction_top [default: 0.45]
      --fraction-bottom <FRACTION_BOTTOM>
          Fraction of vertical border placed on bottom; must sum to 1.0 with --fraction_bottom [default: 0.55]
      --target-ratio <TARGET_RATIO>
          Target aspect ratio (width/height) for the output image [default: 0.8]
      --output-dir <OUTPUT_DIR>
          Directory to write output images into [default: with_borders]
      --quality <QUALITY>
          JPEG output quality (1–100); ignored for non-JPEG formats [default: 100]
      --color <COLOR>
          Border colour as a hex string (e.g. ffffff) [default: ffffff]
  -h, --help
          Print help
```

### Examples

```sh
borders photo1.jpg photo2.jpg
borders --color 000000 --quality 85 --output-dir processed *.jpg
```
