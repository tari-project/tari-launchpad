// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Tabs,
    Frame,
};
use strum::IntoEnumIterator;

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
where
    T: IntoEnumIterator,
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
where
    T: TabGetter,
{
    type Output = ();

    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) -> Option<Self::Output> {
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
        None
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
                Line::from(spans)
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
