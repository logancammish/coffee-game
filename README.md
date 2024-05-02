# coffee-game

Game made in Rust using the Coffee crate. 

A player uses WASD to move around and must avoid hitting the other player, who uses the arrow keys. The player who uses the red keys is much faster but has lower health. The objective is to hit the other play while in control until they die.


Webpage: https://logancammish.github.io/coffeegame 
 



# Installation
[![build-windows](https://github.com/logancammish/coffee-game/actions/workflows/windows.yml/badge.svg)](https://github.com/logancammish/coffee-game/actions/workflows/windows.yml)
[![build-macos](https://github.com/logancammish/coffee-game/actions/workflows/macos.yml/badge.svg)](https://github.com/logancammish/coffee-game/actions/workflows/macos.yml)
[![build-linux](https://github.com/logancammish/coffee-game/actions/workflows/linux.yml/badge.svg)](https://github.com/logancammish/coffee-game/actions/workflows/linux.yml)

### <ins>Windows:</ins>
1. Head over to [here](https://github.com/logancammish/coffee-game/releases/latest) and download your executable

### <ins>Linux:</ins>
> N.B: Linux support requires you have installed a large amount of package prerequsites (on Ubuntu most of these can be found in `build-essential` and `libudev-dev`, however, on some distributions the list is longer)

> N.B: Linux is not supported due to an issue with Coffee when attempting to build for Wayland

1. Install Rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. The following script will create the binary in the /usr/bin directory:
```
#!/bin/bash
git clone https://github.com/logancammish/coffee-game.git
cd coffee-game
cargo build --release
sudo chmod +x ./target/release/game
sudo cp ./target/release/game /usr/bin
cd ..
rm coffee-game
```

### <ins>Others:</ins>
1. Install Rust [here](https://www.rust-lang.org/tools/install) or with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` on Unix-based systems (MacOS, Linux...)
2. Clone this github repository (with git: `git clone https://github.com/logancammish/coffee-game.git`)
3. Open the repository location in a terminal, and run `cargo build --release`
4. You will find your executable in `/target/release`

# Updating
> Note: You can always roll back to an old release by downloading an older release

### <ins>Windows</ins>
1. Remove your executable, wherever it is located
2. Redownload the executable from [here](https://github.com/logancammish/coffee-game/releases/latest) in the same location as the old file

### <ins>Others:</ins>
1. Delete the old clone
2. Follow the installation instructions again
   
# Copyright
Copyright © Logan Cammish 2024

License: MIT

