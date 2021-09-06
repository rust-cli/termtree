# treeline [![Main](https://github.com/softprops/treeline/actions/workflows/main.yml/badge.svg)](https://github.com/softprops/treeline/actions/workflows/main.yml)

> a rust library for visualizing tree like data

[API documentation](https://softprops.github.com/treeline)

## install

Add the following to your `Cargo.toml`

```toml
[dependencies]
treeline = "0.1"
```

## usage

an example program is provided under the "examples" directory to mimic the `tree(1)`
linux program

```bash
$ cargo run --example tree target
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/tree target`
target
└── debug
    ├── .cargo-lock
    ├── .fingerprint
    |   └── treeline-21a5bdbd42e0b6da
    |       ├── dep-example-tree
    |       ├── dep-lib-treeline
    |       ├── example-tree
    |       ├── example-tree.json
    |       ├── lib-treeline
    |       └── lib-treeline.json
    ├── build
    ├── deps
    |   └── libtreeline.rlib
    ├── examples
    |   ├── tree
    |   └── tree.dSYM
    |       └── Contents
    |           ├── Info.plist
    |           └── Resources
    |               └── DWARF
    |                   └── tree
    ├── libtreeline.rlib
    └── native
```

Doug Tangren (softprops) 2017
