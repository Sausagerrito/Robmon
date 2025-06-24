//use crossterm;
use ratatui;
use std;
mod gpu_stats;

fn main() {
    let mut terminal = ratatui::init();

    loop {
        let gpu_vec = gpu_stats::print_gpu();
        terminal
            .draw(|frame| {
                draw(
                    frame, &gpu_vec.0, &gpu_vec.1, &gpu_vec.2, gpu_vec.3, gpu_vec.4, &gpu_vec.5,
                )
            })
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    //ratatui::restore();
}

pub fn draw(
    frame: &mut ratatui::Frame,
    gpu_name: &str,
    core_clock: &u32,
    mem_clock: &u32,
    vram_usage: f64,
    core_usage: u32,
    _bus_usage: &u32,
) {
    let outer_block = ratatui::widgets::Block::default()
        .title(gpu_name)
        .borders(ratatui::widgets::Borders::ALL);
    let area = frame.area();
    let inner_area = outer_block.inner(area);
    frame.render_widget(outer_block, area);

    // Horizontal Layout
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(50),
            ratatui::layout::Constraint::Percentage(50),
        ])
        .split(inner_area);

    // Vertical Splits
    // GPU CORE CLOCK CHART
    // For core_chart:
    let core_bar_width = chunks[0].width.saturating_sub(2); // subtract borders
    let core_chart = ratatui::widgets::BarChart::default()
        .block(
            ratatui::widgets::Block::default()
                .title(format!("C. CLK: {} MHz", core_clock))
                .borders(ratatui::widgets::Borders::ALL),
        )
        .data(&[("LOAD:", u64::from(core_usage))])
        .bar_width(core_bar_width)
        .max(100)
        .bar_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));

    // For mem_chart:
    let mem_bar_width = chunks[1].width.saturating_sub(2); // subtract borders
    let mem_chart = ratatui::widgets::BarChart::default()
        .block(
            ratatui::widgets::Block::default()
                .title(format!("M. CLK: {} MHz", mem_clock))
                .borders(ratatui::widgets::Borders::ALL),
        )
        .data(&[("VRAM:", vram_usage as u64)])
        .bar_width(mem_bar_width)
        .max(100)
        .bar_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue));

    frame.render_widget(core_chart, chunks[0]);
    frame.render_widget(mem_chart, chunks[1]);
}
