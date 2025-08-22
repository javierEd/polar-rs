# Polar client library for Rust

[![crates.io](https://img.shields.io/crates/v/polar-rs.svg)](https://crates.io/crates/polar-rs)
[![docs.rs](https://docs.rs/polar-rs/badge.svg)](https://docs.rs/polar-rs)

> [!WARNING]
> THIS LIBRARY IS IN A VERY EARLY STAGE, PLEASE BE CAREFUL.

## Features

### **Checkouts**

| Description                          | Status |
| ------------------------------------ | ------ |
| Create checkout session              | ✅     |
| Get checkout session                 | ✅     |
| List checkout sessions               | ✅     |
| Update checkout session              | ⏳     |
| Get checkout session from client     | ⏳     |
| Update checkout session from client  | ⏳     |
| Confirm checkout session from client | ⏳     |

### **Subscriptions**

| Description         | Status |
| ------------------- | ------ |
| Get subscription    | ✅     |
| List subscriptions  | ⏳     |
| Update subscription | ✅     |
| Revoke subscription | ✅     |

### Products

| Description             | Status |
| ----------------------- | ------ |
| Get product             | ✅     |
| List products           | ✅     |
| Create product          | ✅     |
| Update product          | ⏳     |
| Update product benefits | ⏳     |

## Getting started

- Installation

```bash
cargo add polar-rs
```

- Example usage

```rust
use polar_rs::Polar;

let polar = Polar::new("https://sandbox-api.polar.sh/v1/", "<YOUR ACCESS TOKEN>");
```
