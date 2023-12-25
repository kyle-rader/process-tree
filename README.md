# Process Tree

A library wrapping [windows-rs](https://crates.io/crates/windows) to provide a `ProcessTree` struct which can get the ancestry of parent processes for the current process. (And in the future, more!)

Example Usage:
```rust
use process_tree::ProcessTree;

fn main() -> anyhow::Result<()> {
    let process_tree = ProcessTree::new()?;
    println!("My parents are: {:#?}", process_tree.parents());
    Ok(())
}
```

Output:
```shell
$ cargo run --example parents
My parents are: [
    Process {
        name: "parents.exe",
        pid: 32628,
        parent: 7396,
    },
    Process {
        name: "cargo.exe",
        pid: 7396,
        parent: 3616,
    },
    Process {
        name: "cargo.exe",
        pid: 3616,
        parent: 21332,
    },
    Process {
        name: "pwsh.exe",
        pid: 21332,
        parent: 20060,
    },
    Process {
        name: "Code.exe",
        pid: 20060,
        parent: 30116,
    },
    Process {
        name: "Code.exe",
        pid: 30116,
        parent: 23248,
    },
    Process {
        name: "Code.exe",
        pid: 23248,
        parent: 25280,
    },
    Process {
        name: "Code.exe",
        pid: 25280,
        parent: 30116,
    },
]
```