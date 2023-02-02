use crate::{app::App, utils::constants::app_constants::QUIT_INFO};
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
    let mut cpu_datasets = Vec::<Dataset>::new();
    let mut color_index = 0;
    for (k, v) in &app.cpu_signal.data {
        color_index += 1;
        cpu_datasets.push(Dataset::default().name(k).marker(symbols::Marker::Braille).style(Style::default().fg(Color::Indexed(color_index))).graph_type(GraphType::Line).data(v));
    }
    let cpu_chart = Chart::new(cpu_datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "CPU Usage",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 40.0]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
                ])
                .bounds([0.0, 100.0]),
        );
    f.render_widget(cpu_chart, area);
}