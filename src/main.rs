//use crossterm;
use ratatui;
use std;
mod gpu_stats;

fn main() {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    //ratatui::restore();
}

pub fn draw(frame: &mut ratatui::Frame) {
    let gpu_vec = gpu_stats::print_gpu();
    let text = format!(
        "GPU: {}    Core Clock: {} mHz    Memory Clock: {} mHz    VRAM Usage: {:.2}%",
        gpu_vec.0, gpu_vec.1, gpu_vec.2, gpu_vec.3
    );

    frame.render_widget(text, frame.area());
}
