Relm4 Issue 641 Reproducer
===

This is a simple reproduction for [Relm4#641](https://github.com/Relm4/Relm4/issues/641).

## Compiler Output
Running `cargo check`:
```text
error[E0425]: cannot find value `value` in this scope
  --> src/main.rs:59:56
   |
59 | ...                   set_label: &format!("This also doesn't: {:?}", value) // ERROR: cannot find value `value` in this scope
   |                                                                      ^^^^^ not found in this scope

error[E0425]: cannot find value `item` in this scope
  --> src/main.rs:49:8
   |
49 | ...                   item { // ERROR: cannot find value `item` in this scope
   |                       ^^^^ not found in this scope

error[E0425]: cannot find value `item` in this scope
  --> src/main.rs:70:6
   |
70 |                     item { // ERROR: cannot find value `item` in this scope
   |                     ^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `relm4-issue-641` (bin "relm4-issue-641") due to 3 previous errors
```

Output is the same when using latest Relm4 git version (`cargo check --no-default-features --features git`)

## Environment Information

| Software | Version                |
|----------|------------------------|
| OS       | Fedora 41 KDE Spin     |
| Kernel   | 6.13.4-200.fc41.x86_64 |
| GTK      | 4.16.5                 |
| Rust     | 1.86.1-beta*           |
| Relm4    | 0.9.1\*\*              |

\**issue also occurs on latest stable (1.85.0)*

\*\**issue also occurs with latest git version 
(commit [0f7e63f](https://github.com/Relm4/Relm4/commit/0f7e63f5aa93c031cc778ff8feb2871a3b5e6a85) at time of writing)*

## License
Licensed as [CC0](https://creativecommons.org/public-domain/cc0/).

![CC0](http://i.creativecommons.org/p/zero/1.0/88x31.png)