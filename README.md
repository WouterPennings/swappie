# Swappie

Swappie is a simple CLI to mirror text. 

## How to run

This is how you cal Swappie: `swappie <flags> <text>`

The flags that you can use are:

- `-v` or `--verbose`, to get better expressed output
- `--allow-mangle`, does not crash when character is unmirrorable

You could run something like this:

- `swappie --verbose "Hello, World!"`

## How to build

1. Clone repository: `git clone https://github.com/WouterPennings/swappie.git`
2. Move into `swappie/`
3. Build project: `cargo build --release`
4. Add the executable to your PATH, executable is located at: `./target/release/swappie<.exe>`