use nvml_wrapper::Nvml;

pub fn print_gpu(){
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let memory_info = device.memory_info().unwrap();
    let mem_used: f64 = memory_info.used as f64;
    let mem_tot: f64 = memory_info.total as f64;


    println!("Memory Usage: {:.4}%", (mem_used/mem_tot));
}