use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use process_tree::ProcessTree;
use windows::Win32::System::{
    Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
    Threading::GetCurrentProcessId,
};

fn main() -> anyhow::Result<()> {
    println!("Hello, processes!");

    let process_tree = ProcessTree::new()?;

    println!("Process tree:\n{:?}", process_tree);

    Ok(())
}
