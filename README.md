# dotago

[![Crates.io](https://img.shields.io/crates/v/dotago.svg)](https://crates.io/crates/dotago)
[![Documentation](https://docs.rs/dotago/badge.svg)](https://docs.rs/emojic)
[![License](https://img.shields.io/github/license/orhanbalci/dotago.svg)](https://github.com/orhanbalci/dotago/blob/master/LICENSE)

<!-- cargo-rdme start -->


This crate is a rustlang port of [dotago.js](https://github.com/alexpusch/dotago.js) npm package written by
[@alexpusch](https://github.com/alexpusch).


### 📦 Cargo.toml

```toml
[dependencies]
dotago = "0.1"
```

### 🔧 Example

#### Work with dates
```rust
now: 2023-03-15T10:00:00Z
1.minute().ago().as_date(); // 2023-03-15T09:59:00Z
2.hours().ago().as_date(); // 2023-03-15T08:00:00Z
3.hours().from_now().as_date(); // 2023-03-15T13:00:00Z
```


<!-- cargo-rdme end -->
