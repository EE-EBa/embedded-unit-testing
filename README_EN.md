# Embedded Unit Testing Template Project

This is a template project for embedded unit testing. During the setup process, I encountered several pitfalls. Here are the main aspects to pay attention to:

## 1. Rust Compiler Flags

The rustc compilation flags need some additional settings. Although not all of them are mandatory, the following two items seem to be required:

```toml
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=-Tdefmt.x",
]
```

## 2. File Structure

The file structure should be organized as follows:

```
.─┬─►.cargo/.config.toml
 │
 ├─►src
 │ ├►lib.rs
 │ ├►main.rs
 │ └►A.rs (Specific implementation)
 ├─►tests
 │ │
 │ └►tested_A.rs (Test platform for A)
 │
 └─►Cargo.toml
```

## 3. Cargo.toml Configuration

The following dependencies need to be included in Cargo.toml:

```toml
[dependencies]
defmt-rtt = "0.4"
defmt = "1.0.1"

[dev-dependencies]
defmt-test = "0.4.0"

[lib]
harness = false # This needs to be set to false, otherwise lib.rs will keep prompting that tests cannot be found

[[test]] # Place the module test platforms in the tests directory and reference them as follows
name = "macro_custom" # tests/macro_custom.rs
harness = false
```

Setting `harness = false` disables Rust's standard library testing framework.

## 4. Adding Test Commands

Before running test commands, you need to use probe-rs. Therefore, the following configuration needs to be added to `.cargo/config.toml`:

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32G431RBTx"
```

## 5. lib.rs setting

If the module under test contains element like Vec that require heap allocation , you need to pre-allocate heap space in lib.rs just like main.rs.
