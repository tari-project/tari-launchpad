#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Focus(pub &'static str);

#[macro_export]
macro_rules! focus_id {
    () => {{
        $crate::state::focus::Focus(concat!(file!(), line!()))
    }};
}

pub static TERMINATION: Focus = focus_id!();
pub static ONBOARDING: Focus = focus_id!();
pub static ROOT: Focus = focus_id!();
pub static TARI_MINING: Focus = focus_id!();
pub static MERGED_MINING: Focus = focus_id!();
pub static BASE_NODE: Focus = focus_id!();
pub static PASSWORD: Focus = focus_id!();
pub static WALLET_CONTAINER: Focus = focus_id!();

pub static LOGS_TABLE: Focus = focus_id!();
pub static CONTAINERS_TABLE: Focus = focus_id!();
