use std::{sync::Arc, thread, time::Duration};

use crate::{
    audio::engine::Audio,
    controller::{
        Controller,
        events::{AudioEvent, ScannerEvent},
        state::AppState,
    },
    errors::AppError,
    scanner::Scanner,
    ui::{
        assets::Assets,
        res_handler::{Event, ResHandler},
        wiremann::Wiremann,
    },
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

        let mut controller = Controller::new(
            cx.new(|_| AppState::default()),
            audio_tx,
            audio_rx,
            scanner_tx,
            scanner_rx,
        );

        thread::spawn(move || audio.run());

        thread::spawn(move || scanner.run());

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
                    cx.set_global(controller.clone());

                    let view = cx.new(|cx| Wiremann::new(cx));

                    cx.new(|cx| {
                        let res_handler = cx.new(|_| ResHandler {});
                        let arc_res = Arc::new(res_handler.clone());
                        let mut controller_resclone = controller.clone();

                        cx.spawn(async move |_, cx| {
                            loop {
                                while let Ok(e) = controller.audio_rx.try_recv() {
                                    arc_res.update(cx, |res_handler, cx| {
                                        res_handler.handle(cx, Event::Audio(e));
                                    });
                                }

                                while let Ok(e) = controller.scanner_rx.try_recv() {
                                    arc_res.update(cx, |res_handler, cx| {
                                        res_handler.handle(cx, Event::Scanner(e));
                                    });
                                }

                                cx.background_executor()
                                    .timer(Duration::from_millis(16))
                                    .await;
                            }
                        })
                        .detach();

                        let view_clone = view.clone();

                        cx.subscribe(&res_handler, move |_, _, event, cx| match event {
                            Event::Audio(event) => controller_resclone.handle_audio_event(cx, event),
                            Event::Scanner(event) => controller_resclone.handle_scanner_event(event),
                        })
                        .detach();

                        Root::new(view, window, cx)
                    })
                },
            )?;

            Ok::<_, AppError>(())
        })
        .detach();
    });

    Ok(())
}
