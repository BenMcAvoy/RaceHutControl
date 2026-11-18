# RaceHutControl

RaceHutControl is an early desktop UI prototype for a sailing race hut control system. The current application is built with Rust and `egui` through `eframe`.

At this stage, it is a class-list setup screen: you can choose from the built-in class names and add or remove them in the interface. Hardware and light control, configuration files, and saving the list between runs are not implemented yet.

## Build and run

You will need a Rust toolchain with Cargo. Clone the repository and start the application with:

```sh
git clone https://github.com/BenMcAvoy/RaceHutControl.git
cd RaceHutControl
cargo run
```

The window currently opens in fullscreen mode. To build an optimized binary, run `cargo build --release`.

## Project status

This is a work in progress and does not yet control physical race hut lights. The UI is a first step toward shaping the setup workflow before connecting it to hardware.
