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

### <ins>Others:</ins>
1. Install Rust [here](https://www.rust-lang.org/tools/install) or with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` on Unix-based systems (MacOS, Linux...)
2. If you are trying to use this on Linux, you need the `libudev-dev` package (`apt install libudev-dev`, `dnf install libudev-dev`, etc)
3. Clone this github repository (with git: `git clone https://github.com/logancammish/coffee-game.git`)
4. Open the repository location in a terminal, and run `cargo build --release`
5. You will find your executable in `/target/release`

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

