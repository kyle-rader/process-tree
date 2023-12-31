use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    os::windows::ffi::OsStringExt,
};

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
    pub name: String,
    pub pid: u32,
    pub parent: u32,
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
        }
    }
}

type ProcessesRaw = HashMap<u32, PROCESSENTRY32W>;

#[derive(Debug)]
pub struct ProcessTree {
    current_pid: u32,
    processes: ProcessesRaw,
}

impl ProcessTree {
    pub fn parents(&self) -> Vec<Process> {
        let mut parents: Vec<Process> = Vec::new();
        let mut current_pid = self.current_pid;
        let mut pids = HashSet::new();

        while let Some(parent) = self.processes.get(&current_pid) {
            parents.push((*parent).into());
            current_pid = parent.th32ParentProcessID;
            if !pids.insert(current_pid) {
                // Loop in process tree.
                // Possible since Windows parent PIDs can be reused.
                break;
            }
        }

        parents
    }

    pub fn new() -> Result<ProcessTree, ProcessTreeError> {
        let mut processes: ProcessesRaw = ProcessesRaw::default();

        #[allow(unused_assignments)]
        let mut current_pid: u32 = 0;

        // First get the current process id.
        unsafe {
            current_pid = GetCurrentProcessId();
        }

        if current_pid == 0 {
            return Err(ProcessTreeError::FailedToGetProcessId);
        }

        #[allow(unused_mut)]
        let mut snapshot: HANDLE;

        // Get the system process snapshot, containing all running processes.
        unsafe {
            match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, current_pid) {
                Ok(handle) => snapshot = handle,
                Err(e) => return Err(e.into()),
            }
        }

        // Rebind as immutable.
        let snapshot = snapshot;

        // define the process entry we will fill in with each Process32FirstW and Process32NextW call.
        let mut proc = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        unsafe {
            Process32FirstW(snapshot, &mut proc)?;
        }

        // Got the first process, now loop through the rest.
        processes.insert(proc.th32ProcessID, proc);

        unsafe {
            while Process32NextW(snapshot, &mut proc).is_ok() {
                processes.insert(proc.th32ProcessID, proc);
            }
        }

        Ok(ProcessTree {
            current_pid,
            processes,
        })
    }
}
