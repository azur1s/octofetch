# octofetch

### Table of Contents

1. [Usage](#usage)
2. [Installation](#installation)
3. [Features](#features)
4. [FAQ](#faq)

A CLI tool for fetching GitHub stats, kinda like `neofetch`

<img src=https://taku.n1ko23.moe/static/attachments/1635502055170-image.png>

# Usage

`octofetch <username>`

with custom config file

`octofetch -c path\to\config <username>`

# Installation

### Windows

1. Get the latest `.exe` file from [here](https://github.com/azur1s/octofetch/releases)
2. Place the file somewhere you can access and open terminal at that directory
3. Do `./octofetch.exe <username>`
4. Add to PATH variables for easier usage (optional)

### Local install with cargo

Run `cargo install --git https://github.com/azur1s/octofetch`

### Arch

[AUR](https://aur.archlinux.org/packages/?O=0&K=octofetch) (Not by me, but usable)

`yay -S octofetch` (the `octofetch` packages seems to get faster update than `octofetch-git`)

### Github (requires Cargo)

1. Clone git repos: `git clone https://github.com/azur1s/octofetch`
2. Change directory: `cd octofetch`
3. Build binary `make build` or `cargo bulid`
4. Change directory to `./target/debug` and then run it!

# Features

fetch github user stats lol

# FAQ

### Why rust? that language can also do the same thing!

Yes, I agree with that but, This is my first project
on rust and I might implement some more feature in the
future too. Pretty proud of it ngl :D
