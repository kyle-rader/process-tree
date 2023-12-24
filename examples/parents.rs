use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use windows::Win32::System::{
    Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
    },
    Threading::GetCurrentProcessId,
};

fn main() -> anyhow::Result<()> {
    println!("Hello, processes!");

    let mut me: u32 = 0;

    unsafe {
        me = GetCurrentProcessId();
    }

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, me)?;

        let mut proc = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        match Process32FirstW(snapshot, &mut proc) {
            Ok(_) => {
                let end_index = proc
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(proc.szExeFile.len());

                println!(
                    "First process id is {} ({})",
                    proc.th32ParentProcessID,
                    OsString::from_wide(&proc.szExeFile[..end_index]).to_string_lossy()
                );
            }
            Err(e) => eprintln!("Error getting first process: {}", e),
        }
    }

    println!("My process id is {}", me);

    Ok(())
}
