# Process Tree

A library wrapping [windows-rs](https://crates.io/crates/windows) to provide a `ProcessTree` struct which can get the ancestry of parent processes for the current process. (And in the future, more!)

## Example Usage:
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
        pid: 31896,
        parent: 1324,
    },
    Process {
        name: "pwsh.exe",
        pid: 1324,
        parent: 1560,
    },
    Process {
        name: "WindowsTerminal.exe",
        pid: 1560,
        parent: 7000,
    },
    Process {
        name: "explorer.exe",
        pid: 7000,
        parent: 6952,
    },
]
```

## Performance
Creating a `ProcessTree` takes a snapshot fo the processes on the system. We only parse the executable names while finding parents.

With an AMD Ryzen 9 3900XT 12-Core CPU @ 3.80 GHz the example CLI can get it's parents in roughly `17ms`.
```
❯ hyperfine .\target\release\examples\parents.exe
Benchmark 1: .\target\release\examples\parents.exe
  Time (mean ± σ):      17.1 ms ±   1.9 ms    [User: 0.1 ms, System: 1.5 ms]
  Range (min … max):    13.7 ms …  23.9 ms    130 runs
```