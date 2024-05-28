# Race Hut Control

A piece of software to control race hut lights for sailing.

## Dependencies

```bash
sudo apt install libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

## Usage

Edit `lights.toml` and put in the BCM values for each pin in/out. (or read the file and connect the lights to the default positions)

Launch the program and it will open in fullscreen. It may be desired to make this autostart for kiosk purposes. 

## Contribution

The repository uses [Conventional Commits](https://conventionalcommits.org) and any contributions should follow the same style.

## Building

```bash
git clone https://github.com/BenMcAvoy/RaceHutControl.git
cd RaceHutControl
cargo build --release
```

## Contact

For any queries please contact me at `ben.mcavoy@tutanota.com`
