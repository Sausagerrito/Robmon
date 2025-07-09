mod cpu_stats;
mod gpu_stats;
use std;
fn main() {
    print!("\x1B[2J"); //clear screen

    loop {
        let clk = cpu_stats::fetch_cpu();
        print!("\x1B[1;1H");
        print_cpu(clk);
        print_gpu();
    }
}

fn print_gpu() {
    let gpu = gpu_stats::fetch_gpu();
    println!("{}", gpu.0);
    println!("C. CLK: {}MHz", gpu.1);
    println!("C. USE: {}%", gpu.4);
    println!("VM. CLK: {}MHz", gpu.2);
    println!("VM. USE: {}%", gpu.3);
    println!();
}

fn print_cpu(usage: u32) {
    let cpu = cpu_stats::poll_com().unwrap();
    println!("{}", cpu.0);
    println!("C. CLK: {}MHz", cpu.1);
    println!("C. USE: {}%", usage);
    println!();
}
