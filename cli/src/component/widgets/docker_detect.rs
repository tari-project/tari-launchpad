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
