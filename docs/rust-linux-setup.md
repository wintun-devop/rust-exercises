### Installing Rust on Linux
- https://rust-lang.org/tools/install/
- https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux
- Run on normal uermode only.Not run on sudo.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Activate changes in current shell
```
source "$HOME/.cargo/env"
```
- Verify installation
```
rustc --version
```
```
cargo --version
```
- Install common components
```
rustup component add rustfmt clippy
```
- Update
```
rustup update
```
