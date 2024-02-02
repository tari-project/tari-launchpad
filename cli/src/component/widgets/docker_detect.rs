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

use ratatui::prelude::*;

use crate::component::{widgets::popup::Popup, Frame};

pub fn is_docker_running() -> bool {
    match std::process::Command::new("docker").arg("version").output() {
        Ok(output) => output.stderr.is_empty(),
        Err(_) => false,
    }
}

pub fn display_docker_notice<B: Backend>(f: &mut Frame<B>, title: &str, msg: &str) {
    let popup_area = Rect {
        x: 4,
        y: 2,
        width: 50,
        height: 8,
    };
    let popup = Popup::default()
        .content(msg)
        .style(Style::new().yellow())
        .title(title)
        .title_style(Style::new().white().bold())
        .border_style(Style::new().red());
    f.render_widget(popup, popup_area);
}

pub fn wait_for_keypress() {
    use std::io::{stdin, Read};
    let mut stdin = stdin();
    let buf: &mut [u8] = &mut [0; 2];
    let _unused = stdin.read(buf).expect("Error reading keypress");
}
