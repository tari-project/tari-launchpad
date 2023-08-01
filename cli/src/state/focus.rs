#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Focus(&'static str);

macro_rules! focus {
    () => {{
        Focus(concat!(file!(), line!()))
    }};
}

pub static ONBOARDING: Focus = focus!();
pub static ROOT: Focus = focus!();
pub static TARI_MINING: Focus = focus!();
pub static MERGED_MINING: Focus = focus!();
pub static BASE_NODE: Focus = focus!();
pub static PASSWORD: Focus = focus!();
