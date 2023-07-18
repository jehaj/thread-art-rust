# thread-art-rust
The art style computor as seen in Petros Vrellis art.

A limitation of the project right know is that the settings must be changed in the source code. `clap` could be used to make runtime changes or that might be over the top for simply reading command line arguments. 

# Usage
To simply run the project use
```
cargo run -r -- [options] <source> <destination>
```
If you want the binary then in the root directory of the repository use the following to build the project in release mode (`-r`)
```
cargo build -r
```
You can find the binary at `target/release/thread-art-rust` if you are using Linux. The name might be something else on other platforms.

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
