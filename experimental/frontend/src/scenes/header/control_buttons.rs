use anyhow::Error;
use tari_launchpad_protocol::launchpad::Reaction;
use yew::{html, Html};

use crate::{
    scenes::{icons, MainView},
    states::{
        local_state::{LocalState, LocalStateDelta, ViewMode, LOCAL_STATE},
        remote_state::RemoteState,
    },
    widget::{Connected, Context, FromDelta, Pod, Widget},
};

pub struct ControlButtons {
    show_icons: bool,
}

#[derive(Clone)]
pub enum Msg {
    ShowIcons(bool),
}

impl FromDelta<LocalState> for Msg {}

impl FromDelta<RemoteState> for Msg {}

impl Widget for ControlButtons {
    type Msg = Msg;

    fn create(_ctx: &mut Context<Self>) -> Self {
        Self { show_icons: false }
    }

    fn on_event(&mut self, event: Msg, ctx: &mut Context<Self>) -> Result<(), Error> {
        match event {
            Msg::ShowIcons(value) => {
                self.show_icons = value;
                ctx.redraw();
            },
        }
        Ok(())
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        Some(html! {
            <div class="control_buttons">
                <button class="close">{ close() }</button>
                <button class="minimize">{ minimize() }</button>
                <button class="maximize">{ maximize() }</button>
            </div>
        })
    }
}

fn close() -> Html {
    // TODO: Move to a separate mod
    html! {
      <svg
        width="8"
        height="8"
        viewBox="0 0 6 6"
        fill="none"
      >
        <path
          d="M4.76796 1.23242L1.23242 4.76796M4.76796 4.76796L1.23242 1.23242"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
        />
      </svg>
    }
}

fn minimize() -> Html {
    html! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="8"
          height="2"
          viewBox="0 0 8 2"
          fill="none"
        >
          <path
            d="M1 1H9"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
          />
        </svg>
    }
}

fn maximize() -> Html {
    html! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="17"
          height="16"
          viewBox="0 0 17 16"
          fill="none"
        >
          <path
            d="M4.04504 4.32699C4.04331 3.99321 4.31434 3.72219 4.64812 3.72391L9.99044 3.75145C10.5235 3.7542 10.7885 4.39878 10.4116 4.77571L5.09683 10.0905C4.7199 10.4674 4.07532 10.2024 4.07257 9.66932L4.04504 4.32699Z"
            fill="currentColor"
          />
          <path
            d="M11.7442 12.0263C12.078 12.028 12.349 11.757 12.3473 11.4232L12.3197 6.08085C12.317 5.5478 11.6724 5.28275 11.2955 5.65968L5.98068 10.9745C5.60376 11.3514 5.86881 11.996 6.40185 11.9987L11.7442 12.0263Z"
            fill="currentColor"
          />
        </svg>
    }
}
