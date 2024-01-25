// Copyright 2023. The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

use ratatui::{
    backend::Backend,
    layout::{Alignment, Rect},
    text::Text,
    widgets::Paragraph,
};

use crate::{
    component::{Component, ComponentEvent, Frame, Input},
    state::AppState,
};

pub struct StatusLine<T> {
    getter: T,
}

pub trait StatusReportGetter {
    fn get_status(&self, state: &AppState) -> Text;
}

impl<T> StatusLine<T> {
    pub fn new(getter: T) -> Self {
        Self { getter }
    }
}

impl<T> Input for StatusLine<T> {
    type Output = ();

    fn on_event(&mut self, _event: ComponentEvent, _state: &mut AppState) -> Option<Self::Output> {
        None
    }
}

impl<B: Backend, T> Component<B> for StatusLine<T>
where
    T: StatusReportGetter,
{
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let text = self.getter.get_status(state);
        let paragraph = Paragraph::new(text).alignment(Alignment::Left);
        f.render_widget(paragraph, rect);
    }
}
