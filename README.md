# Gomoku

GUI frontend for my [gomoku engine](https://github.com/Maneren/gomoku-rust)
built using [Dioxus](https://dioxuslabs.com)

## Installation

Either grab a binary from
[latest release](https://github.com/Maneren/gomoku-gui-dioxus/releases/latest)
or compile it from source

### Compiling

First get [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
and then run

```rs
cargo build --release
```

Resulting binary is `target/release/gomoku`.

On Linux you may have to also install GTK WebView dependencies:

```sh
# Example for Ubuntu based distros
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev
```
