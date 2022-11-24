use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{canvas::{Canvas, Line, Map, MapResolution, Rectangle}, BorderType},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_header(f, app, chunks[0])
}

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM).border_type(BorderType::Double);
    let text = vec![
        Spans::from(app.title)
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}