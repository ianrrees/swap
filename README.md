swap
===

Command line tool for swapping the names of two files.

```
➜  ~ echo apple > file_a.txt
➜  ~ echo banana > file_b.txt
➜  ~ swap file_a.txt file_b.txt
➜  ~ cat file_a.txt
banana
➜  ~ cat file_b.txt
apple
```

This is simply a wrapper around the renameat2 syscall, which is Linux-only.

Building
---
`cargo build`

Installing
---
On on Debian/Ubuntu systems, I'm a fan of cargo-deb, which can be installed via `cargo install cargo-deb`.
Then, `cargo deb --install`.

Contributing
---
I'm happy to accept PRs.