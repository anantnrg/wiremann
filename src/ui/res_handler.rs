use crate::controller::events::{AudioEvent, ScannerEvent};
use gpui::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ResHandlerEvent {
    Audio(AudioEvent),
    Scanner(ScannerEvent),
}

#[derive(Clone, Copy)]
pub struct ResHandler {}

impl ResHandler {
    pub fn handle(&mut self, cx: &mut Context<Self>, event: ResHandlerEvent) {
        cx.emit(event);
        cx.notify();
    }
}

impl EventEmitter<ResHandlerEvent> for ResHandler {}
