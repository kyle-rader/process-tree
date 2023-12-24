use std::{collections::HashMap, ffi::OsString, os::windows::ffi::OsStringExt};

use thiserror::Error;
use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
        Threading::GetCurrentProcessId,
    },
};

#[derive(Debug, Error)]
pub enum ProcessTreeError {
    #[error("Failed to get the current process id")]
    FailedToGetProcessId,

    #[error("WindowsCore: {0}")]
    WindowsCore(#[from] windows::core::Error),
}

#[derive(Debug)]
pub struct Process {
    name: String,
    pid: u32,
    parent: u32,
    children: Vec<u32>,
}

impl From<PROCESSENTRY32W> for Process {
    fn from(proc: PROCESSENTRY32W) -> Self {
        let end_index = proc
            .szExeFile
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(proc.szExeFile.len());

        Process {
            name: OsString::from_wide(&proc.szExeFile[..end_index])
                .to_string_lossy()
                .to_string(),
            pid: proc.th32ProcessID,
            parent: proc.th32ParentProcessID,
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct ProcessTree {
    current_pid: u32,
    processes: HashMap<u32, Process>,
}

impl ProcessTree {
    pub fn new() -> Result<ProcessTree, ProcessTreeError> {
        let mut processes: HashMap<u32, Process> = HashMap::new();

        #[allow(unused_assignments)]
        let mut current_pid: u32 = 0;

        // First get the current process id.
        unsafe {
            current_pid = GetCurrentProcessId();
        }

        if current_pid == 0 {
            return Err(ProcessTreeError::FailedToGetProcessId);
        }

        #[allow(unused_assignments)]
        let mut snapshot: Option<HANDLE> = None;

        // Get the system process snapshot, containing all running processes.
        unsafe {
            match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, current_pid) {
                Ok(handle) => snapshot = Some(handle),
                Err(e) => return Err(e.into()),
            }
        }

        // We have either got the snapshot, or already returned an error.
        let snapshot = snapshot.unwrap();

        // define the process entry we will fill in with each Process32FirstW and Process32NextW call.
        let mut proc = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        unsafe {
            Process32FirstW(snapshot, &mut proc)?;
        }

        // Got the first process, now loop through the rest.
        processes.insert(proc.th32ProcessID, proc.into());

        unsafe {
            while Process32NextW(snapshot, &mut proc).is_ok() {
                processes.insert(proc.th32ProcessID, proc.into());
            }
        }

        // Fill in children vecs
        let pids = processes.keys().copied().collect::<Vec<u32>>();
        for pid in pids {
            let parent_pid = processes.get(&pid).unwrap().parent;

            if let Some(parent) = processes.get_mut(&parent_pid) {
                parent.children.push(pid);
            }
        }

        Ok(ProcessTree {
            current_pid,
            processes,
        })
    }
}
