# Amazing Rust
This repository is a Rust port of the code used in the book: Mazes for Programmers.

Some of the methods used or created have been intentionally omitted.

## Installing and running locally
This is a Rust project thus a working Rust installation is required. The code
used requires the Rust 2018 edition, or simply version 1.33 or newer.

```sh
git clone https://github.com/hipstermojo/amazing-rust.git
cd amazing-rust/
cargo run
```

## Note
For the rendering and creation of images, the project uses cairo2, which must be installed
separately. Please refer to their documentation on installation procedures.
For Ubuntu users, run the following command:
```sh
sudo apt-get libcairo2-dev
```