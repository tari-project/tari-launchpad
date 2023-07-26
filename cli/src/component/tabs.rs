use strum::IntoEnumIterator;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Tabs,
    Frame,
};

use crate::{
    component::{elements::block_with_title, Component, ComponentEvent, Input, Pass},
    state::{focus, AppState, Focus},
};

pub trait TabGetter: IntoEnumIterator + Copy + ToString {
    fn get_badge(&self, _: &AppState) -> Option<(&str, Color)> {
        None
    }

    fn focus_to(&self, _: &AppState) -> Focus;
}

pub struct AppTabs<T> {
    focus_on: Focus,
    selected: usize,
    items: Vec<T>,
}

impl<T> AppTabs<T>
where T: IntoEnumIterator
{
    pub fn new() -> Self {
        Self {
            focus_on: focus::ROOT,
            selected: 0,
            items: T::iter().collect(),
        }
    }
}

impl<T> AppTabs<T> {
    pub fn selected(&self) -> &T {
        self.items
            .get(self.selected)
            .expect("the selected tab is out of the range (empty tabs list)")
    }

    fn next(&mut self) {
        let index = self.selected + 1;
        if self.items.get(index).is_some() {
            self.selected = index;
        } else {
            self.selected = 0;
        }
    }

    fn prev(&mut self) {
        if self.selected > 0 {
            let index = self.selected - 1;
            self.selected = index;
        } else {
            self.selected = self.items.len() - 1;
        }
    }
}

impl<T> Input for AppTabs<T>
where T: TabGetter
{
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        if state.focus_on == self.focus_on {
            match event.pass() {
                Pass::Next | Pass::Right => {
                    self.next();
                },
                Pass::Left => {
                    self.prev();
                },
                Pass::Down | Pass::Enter | Pass::Space => {
                    let focus_to = self.selected().focus_to(state);
                    state.focus_on(focus_to);
                },
                _ => {},
            }
        }
    }
}

impl<B, T> Component<B> for AppTabs<T>
where
    B: Backend,
    T: TabGetter,
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let titles = self
            .items
            .iter()
            .map(|item| {
                //.fg(Color::Rgb(4, 209, 144));
                let mut spans = vec![Span::raw(item.to_string())];
                if let Some((tag, color)) = item.get_badge(state) {
                    let tag_style = Style::default().fg(color);
                    let text = format!(" {tag}");
                    let span = Span::styled(text, tag_style);
                    spans.push(span);
                }
                Spans::from(spans)
            })
            .collect();
        let block = block_with_title(None, state.focus_on == self.focus_on);
        let tabs = Tabs::new(titles)
            .block(block)
            .select(self.selected)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Magenta));
        f.render_widget(tabs, rect);
    }
}
