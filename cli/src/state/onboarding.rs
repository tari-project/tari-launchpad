#[derive(Debug, Clone)]
pub struct Progress {
    pub activity: Option<String>,
    pub pct: u8,
}

impl Progress {
    pub fn new() -> Self {
        Self { activity: None, pct: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Onboarding {
    pub history: Vec<Message>,
    pub message: Option<Message>,
    pub total_progress: Progress,
    pub local_progress: Option<Progress>,
}

impl Onboarding {
    pub fn update(&mut self, delta: OnboardingDelta) {
        match delta {
            OnboardingDelta::Add(msg) => {
                if let Some(message) = self.message.take() {
                    self.history.push(message);
                }
                self.message = Some(msg);
            },
            OnboardingDelta::SetProgress(pct) => {
                self.total_progress.pct = pct;
            },
        }
    }

    pub fn is_done(&self) -> bool {
        self.total_progress.pct == 100
    }
}

impl Default for Onboarding {
    fn default() -> Self {
        Self {
            history: Vec::new(),
            message: None,
            total_progress: Progress::new(),
            local_progress: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OnboardingAction {
    Next,
}

#[derive(Debug, Clone)]
pub enum OnboardingDelta {
    Add(Message),
    SetProgress(u8),
}
