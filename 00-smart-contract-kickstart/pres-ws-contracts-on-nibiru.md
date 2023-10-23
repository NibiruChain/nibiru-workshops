# Nibiru Chain: Technical Workshop

<!-- class: invert -->

![bg](./assets/bg-dark.png)
<!-- Changes background of the first slide. -->

---

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Installing Rust using `rustup` will also install `cargo`, which is used to:

1. Build and run Rust programs
2. Run tests
3. Manage crate depencencies. 
4. Publish crates

Learn more in [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/first-steps.html).

---

## We'll use `just` (justfile) instead of `make` (Makefile) because it's better.

1. 
    ```bash
    cargo install just
    ```
2. Then, run `just` in the repo to see some documentation on each command.

---

## Install Everything Else

```bash 
just install-everything
```

--- 

## Install Everything Else

```bash 
just install-everything
```

Under the hood, this will run the following commands automatically.

```bash
# Install WebAssembly target
rustup target add wasm32-unknown-unknown
```

```bash
# Download `nibid` and `pricefeeder`
curl -s https://get.nibiru.fi/! | bash
curl -s https://get.nibiru.fi/pricefeeder! | bash
```

---



--- 

## Compiling contracts

- [ ] just wasm-all
- [ ] just tidy 


--- 

Instructions to install Go:
https://go.dev/doc/install

https://get.nibiru.fi/
https://get.nibiru.fi/pricefeeder

https://get.nibiru.fi/?type=script
https://get.nibiru.fi/pricefeeder?type=script

Local Network

- [ ] Run the network

---

What happens?
- Chain running with a single validator node
- The mnemonic to use this account is known
- The account is bootstrapped with large amounts of funds