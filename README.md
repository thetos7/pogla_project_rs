# POGLA project
Authors: Ancelin BOUCHET & Thibault AMBROSINO

Particles (round 2!)

This project implements a particle engine using OpenGL's compute and geometry shaders.

TODO: Actually implement the particles

## Building

This project require that you have the sdl2 development libraries installed, if not you can either:
- Install them, ubuntu example:
    ```sh
    sudo apt install libsdl2-dev
    ```
- Change project to use the bundled feature, for that modify the Cargo.toml file from
  ```toml
  sdl2 = "versionString"
  ```
  to
  ```toml
  sdl2 = { version = "versionString", features = ["bundled"]}
  ```

For mor info, check the rust sdl2 crate's readme at https://github.com/Rust-SDL2/rust-sdl2.

To build the program, use the `cargo build` command, the build result will be situated in the `target/debug` directory. To build on release mode (added optimisation flags), use the `--release` flag, build results will be in `target/release`
```sh
cargo build # -> target/debug
cargo build --release # -> target/release
```

To run the program, you can use `cargo run` (similarly with the `--release` flag for release mode). To pass CLI arguments to the program, you need to use `--` before the program's arguments.
```sh
cargo run # runs in debug mode
cargo run --release # runs in release mode
cargo run -- --arg # passes --arg to the program
```

Each execution of either `cargo build` or `cargo run` will download and build dependencies if needed.

Once you are done, run `cargo clean` to cleanup.

## CLI

You can access the CLI's help by passing the `-h` or `--help` argument.

You can set the logging level during execution by setting the `LOG_LEVEL` environment variable to one of `trace`, `debug`, `info`, `warn`, `error`, or `off` (all case insensitive). More detailed explanations at https://docs.rs/env_logger/latest/env_logger/.


## Controls

- `Mouse movements`: Look around
- `Z` `Q` `S` `D`: Horizontal movement
- `Space`: Move up
- `Left Shift`: Move down
- `Tab`: Toggle cursor capture
- `Left click` on window: Enable cursor capture
- `Escape`: Close application
- `B`: Toggle broken capture fix
- `L`: Log debug info

