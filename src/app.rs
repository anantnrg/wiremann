use std::thread;

use crate::{
    audio::engine::Audio,
    controller::{
        Controller,
        state::{AppState, AppStateGlobal},
    },
    errors::AppError,
    scanner::Scanner,
    ui::{assets::Assets, commander::UiCommander, wiremann::Wiremann},
};
use gpui::*;
use gpui_component::*;

pub fn run() -> Result<(), AppError> {
    let assets = Assets {};
    let app = Application::new().with_assets(assets.clone());

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1280.0), px(760.0)), cx);
        assets.load_fonts(cx).expect("Could not load fonts");

        let (mut audio, audio_tx, audio_rx) = Audio::new();
        let (mut scanner, scanner_tx, scanner_rx) = Scanner::new();
        let (ui_cmd_tx, ui_cmd_rx) = crossbeam_channel::unbounded();
        let (ui_event_tx, ui_event_rx) = crossbeam_channel::unbounded();

        thread::spawn(move || audio.run());

        thread::spawn(move || scanner.run());

        let state = cx.new(|_| AppState::default());
        cx.set_global(AppStateGlobal(state.clone()));
        cx.set_global(UiCommander(ui_cmd_tx));

        let controller = Controller::new(
            state,
            audio_tx,
            audio_rx,
            scanner_tx,
            scanner_rx,
            ui_cmd_rx,
            ui_event_tx,
        );

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    app_id: Some(String::from("wiremann")),
                    focus: true,
                    titlebar: Some(TitlebarOptions {
                        title: None,
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_min_size: Some(gpui::Size {
                        width: px(800.0),
                        height: px(740.0),
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Wiremann::new(cx));

                    cx.spawn(async move |_| {}).detach();

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, AppError>(())
        })
        .detach();
    });

    Ok(())
}
