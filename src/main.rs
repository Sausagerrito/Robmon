use crossterm;
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
        if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap() {
                if key_event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    crossterm::terminal::disable_raw_mode().unwrap();
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )
    .unwrap();
}

pub fn draw(
    frame: &mut ratatui::Frame,
    gpu_name: &str,
    core_clock: &u32,
    mem_clock: &u32,
    vram_usage: u64,
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
    let core_bar_width = chunks[0].width.saturating_sub(2);
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

    let mem_bar_width = chunks[1].width.saturating_sub(2);
    let mem_chart = ratatui::widgets::BarChart::default()
        .block(
            ratatui::widgets::Block::default()
                .title(format!("M. CLK: {} MHz", mem_clock))
                .borders(ratatui::widgets::Borders::ALL),
        )
        .data(&[("VRAM:", vram_usage)])
        .bar_width(mem_bar_width)
        .max(100)
        .bar_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue));

    frame.render_widget(core_chart, chunks[0]);
    frame.render_widget(mem_chart, chunks[1]);
}
