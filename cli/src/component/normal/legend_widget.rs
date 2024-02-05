// Copyright 2024. The Tari Project
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

use crate::component::elements::block_with_title;
use crate::component::Component;
use crate::state::AppState;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Line;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{List, ListItem, Padding};
use ratatui::Frame;

pub struct LegendWidget {}

impl LegendWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl<B: Backend> Component<B> for LegendWidget {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, _state: &Self::State) {
        let block = block_with_title(Some("Key Commands"), false).padding(Padding::new(1, 1, 1, 1));
        let inner_rect = block.inner(rect);
        f.render_widget(block, rect);

        let command_items = [
            ["Crtl-Q", "Quit"],
            ["Crtl-B", "Start Tari node"],
            ["Crtl-H", "Switch to home"],
            ["Crtl-L", "Switch to logs"],
            ["Crtl-S", "Switch to settings"],
            ["M     ", "Merge mine"],
            ["T     ", "Sha mine"],
            ["Enter ", "Edit/stop editing field"],
            ["Left-Arrow ", "Move left"],
            ["Right-Arrow", "Move right"],
            ["Up-Arrow   ", "Move up"],
            ["Down-Arrow ", "Move down"],
        ];

        let items_per_column = (command_items.len() as f32 / 3.0).ceil() as usize;

        // Split the items into three columns
        let column1_items = command_items.iter().take(items_per_column).copied().collect::<Vec<_>>();
        let column2_items = command_items
            .iter()
            .skip(items_per_column)
            .take(items_per_column)
            .copied()
            .collect::<Vec<_>>();
        let column3_items = command_items
            .iter()
            .skip(2 * items_per_column)
            .copied()
            .collect::<Vec<_>>();

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(inner_rect);

        // Render each column as a List
        for (i, columns) in [column1_items, column2_items, column3_items].iter().enumerate() {
            f.render_widget(
                List::new(
                    columns
                        .iter()
                        .map(|&item| {
                            ListItem::new(Line::from(vec![
                                Span::styled(format!("{} ", item[0]), Style::default().fg(Color::Blue)),
                                Span::from(item[1]),
                            ]))
                        })
                        .collect::<Vec<ListItem>>(),
                ),
                layout[i],
            );
        }
    }
}
