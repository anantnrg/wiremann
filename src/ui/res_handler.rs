use crate::controller::events::{AudioEvent, ScannerEvent, UiEvent};
use gpui::*;

#[derive(Clone, Copy)]
pub struct ResHandler {}

impl ResHandler {
    pub fn handle(&mut self, cx: &mut Context<Self>, event: UiEvent) {
        cx.emit(event);
        cx.notify();
    }
}

impl EventEmitter<UiEvent> for ResHandler {}
