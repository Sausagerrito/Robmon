use nvml_wrapper::Nvml;

pub fn print_gpu() -> (String, u32, u32, f64, u32, u32) {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let memory_info = device.memory_info().unwrap();
    let mem_used: f64 = memory_info.used as f64;
    let mem_tot: f64 = memory_info.total as f64;
    let mem = 100.00 * (mem_used / mem_tot);

    let gpu_name = device.name().unwrap();
    let gpu_clock = device
        .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)
        .unwrap();
    let mem_clock = device
        .clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory)
        .unwrap();
    let gpu_usage = device.utilization_rates().unwrap();
    let core_usage = gpu_usage.gpu;
    let bus_usage = gpu_usage.memory;
    (gpu_name, gpu_clock, mem_clock, mem, core_usage, bus_usage)
}
