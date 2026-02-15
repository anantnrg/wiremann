use crate::controller::commands::UiCommand;
use crossbeam_channel::Sender;
use gpui::Global;

pub struct UiCommander(pub Sender<UiCommand>);

impl Global for UiCommander {}