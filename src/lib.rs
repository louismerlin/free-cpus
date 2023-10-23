//! This crate exports a utility function that returns the set of available core IDs on the machine.
//!
//! The set is determined by iterating over the `/proc` directory's process statuses.
//! The logic is heavily inspired by [AFL++'s code](https://github.com/AFLplusplus/AFLplusplus/blob/85c5b5218c6a7b2289f309fbd1625a5d0a602a00/src/afl-fuzz-init.c#L109-L452).
use std::{
    collections::HashSet,
    fs::{read_dir, File},
    io::{BufRead, BufReader},
};

/// Returns the set of available core IDs.
///
/// Logic inspired by [AFL++'s code](https://github.com/AFLplusplus/AFLplusplus/blob/85c5b5218c6a7b2289f309fbd1625a5d0a602a00/src/afl-fuzz-init.c#L109-L452)
#[cfg(target_os = "linux")]
pub fn get() -> HashSet<usize> {
    let mut cpu_used = HashSet::new();
    let proc = read_dir("/proc").expect("Could not read /proc, we cannot safely assign cores");
    // We iterate over every directory's status file in /proc
    for status in proc
        .flatten()
        .filter(|p| p.metadata().map(|p| p.is_dir()).unwrap_or(false))
        .filter_map(|p| p.file_name().to_str().map(String::from))
        .flat_map(|pid| File::open(format!("/proc/{pid}/status")))
        .map(BufReader::new)
    {
        let mut has_vmsize = false;
        for line in status.lines().flatten() {
            if line.contains("VmSize:\t") {
                has_vmsize = true;
            }
            if has_vmsize
                && line.contains("Cpus_allowed_list:\t")
                && !line.contains('-')
                && !line.contains(',')
            {
                if let Some(id_str) = line.strip_prefix("Cpus_allowed_list:\t") {
                    if let Ok(id) = id_str.parse::<usize>() {
                        cpu_used.insert(id);
                    }
                }
            }
        }
    }
    let cores = HashSet::from_iter(0..num_cpus::get());
    HashSet::from_iter(cores.difference(&cpu_used).copied())
}

#[cfg(not(target_os = "linux"))]
pub fn get() -> HashSet<usize> {
    unimplemented!("free-cpus is only implemented for Linux");
}
