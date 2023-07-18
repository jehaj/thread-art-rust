# thread-art-rust
The art style computor as seen in Petros Vrellis art.

# Usage
To simply run the project, use
```
cargo run -r -- [options] <source> <destination>
```
If you want the binary, then in the root directory of the repository use the following to build the project in release mode (`-r`)
```
cargo build -r
```
You can find the binary at `target/release/thread-art-rust` if you are using Linux. There might be a file extension (e.g. `.exe`) on other platforms.

You use it with
```
./thread-art-rust [options] <source> <destination>
```

The options are
- `--size -s` (image size `400`),
- `--wraps -w` (the number of thread wraps `2500`),
- `--points -p` (points on circle `235`),
- `--minimum_difference -m` (points inbetween next choice `20`),
- `--brightness_factor -b` (the amount a pixel is brightnened by `51`).
