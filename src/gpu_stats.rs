use nvml_wrapper::Nvml;

pub fn fetch_gpu() -> (String, u32, u32, u64, u32, u32) {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let mem_used = device.memory_info().unwrap().used;
    let mem_tot = device.memory_info().unwrap().total;
    let mem = ((mem_used as f64 / mem_tot as f64) * 100.0).round() as u64;

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
