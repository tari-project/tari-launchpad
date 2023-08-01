use anyhow::Error;
use async_trait::async_trait;
use tact::{Actor, ActorContext, Do, Task};

use crate::state::{
    bus::Bus,
    onboarding::{Message, OnboardingAction, OnboardingDelta},
};

enum State {
    Empty,
    Welcome,
    Description,
    LetStart,
    Done,
}

impl State {
    fn text(&self) -> &str {
        match self {
            Self::Empty => "",
            Self::Welcome => MSG_1,
            Self::Description => MSG_2,
            Self::LetStart => MSG_3,
            Self::Done => "",
        }
    }

    fn progress(&self) -> u8 {
        match self {
            Self::Empty => 0,
            Self::Welcome => 10,
            Self::Description => 20,
            Self::LetStart => 30,
            Self::Done => 100,
        }
    }
}

pub struct OnboardingWorker {
    bus: Bus,
    actions: Option<Task>,
    state: State,
}

impl OnboardingWorker {
    pub fn new(bus: Bus) -> Self {
        Self {
            bus,
            actions: None,
            state: State::Empty,
        }
    }
}

#[async_trait]
impl Actor for OnboardingWorker {
    async fn initialize(&mut self, ctx: &mut ActorContext<Self>) -> Result<(), Error> {
        let task = self.bus.actions(ctx.recipient());
        self.actions = Some(task);
        Ok(())
    }
}

#[async_trait]
impl Do<OnboardingAction> for OnboardingWorker {
    type Error = Error;

    async fn handle(&mut self, event: OnboardingAction, _ctx: &mut ActorContext<Self>) -> Result<(), Self::Error> {
        match event {
            OnboardingAction::Next => {
                self.next_step();
            },
        }
        Ok(())
    }
}

impl OnboardingWorker {
    fn next_step(&mut self) {
        let next = match self.state {
            State::Empty => State::Welcome,
            State::Welcome => State::Description,
            State::Description => State::LetStart,
            State::LetStart => State::Done,
            State::Done => State::Done,
        };
        self.state = next;
        let text = self.state.text();
        let msg = Message { text: text.into() };
        let delta = OnboardingDelta::Add(msg);
        self.bus.update(delta);
        let progress = self.state.progress();
        let delta = OnboardingDelta::SetProgress(progress);
        self.bus.update(delta);
    }
}

const MSG_1: &str = "
Hi! My name is T-Bot. It is a great pleasure and an honor to meet you!
I have no memory of human faces, so if our paths have already crossed in the Aurora app, I’m glad to see you again!
";

const MSG_2: &str = "
I'm kind of like Gandalf, Dumbledore or Obi-Wan Kenobi. You know, the guy who makes sure the novice gets to a certain \
                     destination. Spoiler alert: in this saga the guide will survive. Regardless of whether this is \
                     your first contact with cryptocurrencies or you are advanced in it, I will stay with you until \
                     the Tari Launchpad setup process is successfully completed.
";

const MSG_3: &str = "
So let's get started! 🚀 The setup process usually takes 5 to 10 minutes. A duo like you and me should be able to deal \
                     with it quickly, right?
";
