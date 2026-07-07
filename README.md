# uuid-lite2

A minimal random (version 4) UUID generator, depending only on [`getrandom`].

## Usage

```rust
let id = uuid_lite2::Uuid::new_v4();
println!("{id}"); // e.g. 67e55044-10b1-426f-9247-bb680e5fe0c8
```

## API

- `Uuid::new_v4()` — generates a random UUID.
- `Uuid::as_bytes()` — returns the underlying `&[u8; 16]`.
- `Display` / `Debug` — formats as the canonical 8-4-4-4-12 hexadecimal string.

## License

MIT

[`getrandom`]: https://crates.io/crates/getrandom
