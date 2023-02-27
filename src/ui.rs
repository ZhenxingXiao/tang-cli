use crate::{app::App, utils::constants::{app_constants::QUIT_INFO, ui_constants::{CPU_USAGE, MIN_USAGE, MAX_USAGE, MEMORY_USAGE}}};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect, Alignment},
    text::{Spans, Span},
    widgets::{BorderType, Chart, Axis, GraphType},
    widgets::{Block, Borders, Paragraph, Wrap, Dataset},
    Frame, style::{Color, Style, Modifier}, symbols,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3)].as_ref())
        .split(f.size());
    draw_header(f, app, chunks[0]);
    draw_content(f, app, chunks[1]);
    draw_footer(f, chunks[2]);
}

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM).border_type(BorderType::Double);
    let text = vec![
        Spans::from(app.title)
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true }).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

pub fn draw_footer<B: Backend>(f: &mut Frame<B>, area: Rect){
    let text: Vec<Spans> = vec![
        Spans::from(QUIT_INFO)
    ];
    let block: Block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM).border_type(BorderType::Plain);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

pub fn draw_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect){
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(area);
    draw_cpu_usage_chart(f, app, chunks[0]);
    draw_mem_usage_chart(f, app, chunks[1]);

}

pub fn draw_usage_chart<B: Backend>(f: &mut Frame<B>, area: Rect, data: &[(f64, f64)], time_range: [f64; 2], title: &str){
    let mut datasets = Vec::<Dataset>::new();
    datasets.push(
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::White))
            .graph_type(GraphType::Line)
            .data(data)
    );
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    title,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(time_range),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled(MIN_USAGE, Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(MAX_USAGE, Style::default().add_modifier(Modifier::BOLD)),
                ])
                .bounds([0.0, 100.0]),
        );
    f.render_widget(chart, area);
}

pub fn draw_mem_usage_chart<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect){
    draw_usage_chart(f, area, &app.mem_signal.mem_total_usage, app.mem_signal.time_range, MEMORY_USAGE);
}

pub fn draw_cpu_usage_chart<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect){
    draw_usage_chart(f, area, &app.cpu_signal.cpu_total_usage, app.cpu_signal.time_range, CPU_USAGE);
}