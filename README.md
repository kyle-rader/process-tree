# Process Tree

A library wrapping [windows-rs](https://crates.io/crates/windows) to provide a `ProcessTree` struct which can get the ancestry of parent processes for the current process. (And in the future, more!)

Example Usage:
```rust
use process_tree::ProcessTree;

fn main() -> anyhow::Result<()> {
    let process_tree = ProcessTree::new()?;
    println!("My ancestors are: {:#?}", process_tree.ancestry());
    Ok(())
}
```

Output:
```shell
$ cargo run --example parents
Hello, processes!
My ancestors are: [
    Process {
        name: "parents.exe",
        pid: 19320,
        parent: 996,
        children: [],
    },
    Process {
        name: "cargo.exe",
        pid: 996,
        parent: 18632,
        children: [
            19320,
        ],
    },
    Process {
        name: "cargo.exe",
        pid: 18632,
        parent: 9032,
        children: [
            996,
        ],
    },
    Process {
        name: "pwsh.exe",
        pid: 9032,
        parent: 6876,
        children: [
            18632,
        ],
    },
    Process {
        name: "Code.exe",
        pid: 6876,
        parent: 3372,
        children: [
            9032,
            4080,
        ],
    },
    Process {
        name: "Code.exe",
        pid: 3372,
        parent: 17064,
        children: [
            17736,
            15960,
            17268,
            6876,
            3728,
            16352,
            5384,
            14804,
        ],
    },
]
```