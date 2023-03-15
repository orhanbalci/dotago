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
use dotago::Dotago;
// now: 2023-03-15T10:00:00Z
1.minute().ago().as_date(); // 2023-03-15T09:59:00Z
2.hours().ago().as_date(); // 2023-03-15T08:00:00Z
3.hours().from_now().as_date(); // 2023-03-15T13:00:00Z
```

#### Work with timestamps
```rust
use dotago::Dotago;
// now: 2020-06-18T10:00:00Z
1.minute().ago(); // 1592474340000
1.hour().ago(); // 1592470800000
1.hour().from_now(); // 1592478000000
```

#### Duration in milliseconds
```rust
use dotago::Dotago;
1.second(); // 1000
1.minute(); // 6000
1.hour(); // 3600000
```

#### Do math
```rust
use dotago::Dotago;
// now: 2020-06-18T10:00:00Z
(1.hour().ago() + 2.minutes()).as_date(); // 2020-06-18T09:02:00Z
```

<!-- cargo-rdme end -->
