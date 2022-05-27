Bug found on aarch64 (Jetson Nano)

```
make
cargo +nightly run --release
```

```
C struct: a = 17767, b = 9158, c = 39017
C function returns 9158
[src/main.rs:10] value = 9158
[src/main.rs:11] value as usize = 846930886
[src/main.rs:12] usize::from(value) = 846930886
[src/main.rs:13] (value as usize) & 0xFFFF = 846930886
```
