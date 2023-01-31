use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect, Alignment},
    text::Spans,
    widgets::BorderType,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
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
        Spans::from("💻 Press 'Q' to quit !")
    ];
    let block: Block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM).border_type(BorderType::Plain);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

pub fn draw_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect){
    
}