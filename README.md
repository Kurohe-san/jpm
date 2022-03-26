# JPM (Just another package manager)
A simple package manager written in Rust. JPM can install packages that are written in a simple JSON format.

## Building
`git clone https://github.com/Kurohe-san/jpm`<br>
`cargo build --release`

## Usage
`jpm <command> <package> [<additional flags>]`

## Features
- [x] Installing packages
- [x] Install dependencies recursively
- [x] Removing packages
- [x] Remove all dependencies of package
- [x] Configure system paths used by JPM
- [x] Search packages in local packags database
- [ ] Update local package sync from mirror
- [ ] List installed packages
- [ ] Upgrade packages via version difference detection