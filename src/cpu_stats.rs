use serde::Deserialize;
use std::time::Duration;
use windows::Win32::Foundation::FILETIME;
use windows::Win32::System::Threading;
use wmi::{COMLibrary, WMIConnection};

#[derive(Debug, Deserialize)]
struct Win32_Processor {
    Name: String,
    CurrentClockSpeed: u32,
}

pub fn fetch_cpu() -> u32 {
    let (idle1, kernel1, user1) = fetch_times();
    std::thread::sleep(Duration::from_millis(500));
    let (idle2, kernel2, user2) = fetch_times();

    let idelta: u64 = ft64(idle2) - ft64(idle1);

    let kdelta: u64 = ft64(kernel2) - ft64(kernel1);

    let udelta: u64 = ft64(user2) - ft64(user1);

    let usage: f64 =
        100.0 * (kdelta as f64 + udelta as f64 - idelta as f64) / (kdelta as f64 + udelta as f64);

    usage as u32
}

fn fetch_times() -> (FILETIME, FILETIME, FILETIME) {
    unsafe {
        let mut idle = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        Threading::GetSystemTimes(Some(&mut idle), Some(&mut kernel), Some(&mut user));

        (idle, kernel, user)
    }
}

fn ft64(ft: FILETIME) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

pub fn poll_com() -> Option<(String, u32)> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
    let result: Vec<Win32_Processor> = wmi_con.query().unwrap();
    let cpu = result.into_iter().next().unwrap();
    Some((cpu.Name, cpu.CurrentClockSpeed))
}
