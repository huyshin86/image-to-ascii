# 🖼️ Image-to-ASCII Converter
A Rust‑based CLI tool that transforms images into ASCII art, either for direct terminal display or as a rendered image file. Supports resizing and adjustable font size for ASCII art.
# 🏗️ Build and Run
### Prerequisites
Rust (includes Cargo) installed.
```bash
rustc --version
cargo --version
```
### Build
```bash
# Debug build (faster to compile, larger binary)
cargo build

# or Optimized release build
cargo build --release
```
The compiled binary will be in:
- `target/debug/your_binary_name` (debug)
- `target/release/your_binary_name` (release)
### Run
```bash
# From project root
cargo run -- --image-path example.png
```
Or
```bash
./target/release/your_binary_name --image-path example.png
```
# ⚙️ Program Arguments
| Argument | Short | Default | Description | Required |
|---|---|---|---|:---:|
|`--input-path`|-i||Path to input image file.|✅|
|`--output-path`|-o|`output.png`|Path to output image file.||
|`--no-resize`||`false`|Disable resizing of the image before ASCII conversion.||
|`--to-terminal`|-t|`false`|Output ASCII art directly to the terminal (conflicts with `--output-path`, `--no-resize`, `--font-size`).||
|`--font-size`||`8.0`|Font size for image output (must be ≥ `1.0`).||
