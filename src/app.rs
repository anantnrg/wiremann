use crossbeam_channel::unbounded;
use std::sync::Arc;
use std::{thread, time::Duration};

use crate::audio::engine::{AudioEngine, PlaybackState};
use crate::controller::metadata::Metadata;
use crate::controller::player::{AudioCommand, AudioEvent, Controller, PlayerState, ResHandler, ScannerCommand, ScannerEvent};
use crate::scanner::Scanner;
use crate::ui::assets::Assets;
use crate::ui::wiremann::Wiremann;
use gpui::*;
use gpui_component::*;

pub fn run() {
    let (audio_cmd_tx, audio_cmd_rx) = unbounded::<AudioCommand>();
    let (audio_events_tx, audio_events_rx) = unbounded::<AudioEvent>();
    let (scanner_cmd_tx, scanner_cmd_rx) = unbounded::<ScannerCommand>();
    let (scanner_events_tx, scanner_events_rx) = unbounded::<ScannerEvent>();

    thread::spawn(move || {
        AudioEngine::run(audio_cmd_rx, audio_events_tx);
    });

    thread::spawn(move || {
        Scanner::run(scanner_cmd_rx, scanner_events_tx);
    });


    let controller = Controller::new(audio_cmd_tx, audio_events_rx, scanner_cmd_tx, scanner_events_rx, PlayerState::default());

    let assets = Assets {};
    let app = Application::new().with_assets(assets.clone());

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1280.0), px(760.0)), cx);
        assets.load_fonts(cx).expect("Could not load fonts");

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
                        width: px(600.0),
                        height: px(400.0),
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let controller_evt_clone = controller.clone();

                    cx.set_global(controller);

                    let view = cx.new(|cx| Wiremann::new(cx));

                    cx.new(|cx| {
                        let res_handler = cx.new(|_| ResHandler {});
                        let arc_res = Arc::new(res_handler.clone());
                        cx.spawn(async move |_, cx| {
                            let res_handler = arc_res.clone();
                            loop {
                                while let Ok(event) = controller_evt_clone.audio_events_rx.try_recv() {
                                    res_handler.update(&mut cx.clone(), |res_handler, cx| {
                                        res_handler.handle(cx, event);
                                    });
                                }
                                cx.background_executor()
                                    .timer(Duration::from_millis(100))
                                    .await;
                            }
                        })
                        .detach();

                        let playbar_view = view.clone();

                        cx.subscribe(
                            &res_handler,
                            move |_, _, event: &AudioEvent, cx| match event {
                                AudioEvent::StateChanged(state) => {
                                    cx.global_mut::<Controller>().state = state.clone();

                                    if state.state == PlaybackState::Playing {
                                        playbar_view.update(cx, |this, cx| {
                                            this.controlbar.update(cx, |this, cx| {
                                                this.playback_slider_state.update(
                                                    cx,
                                                    |this, cx| {
                                                        if let Some(meta) = cx
                                                            .global::<Controller>()
                                                            .state
                                                            .meta
                                                            .clone()
                                                        {
                                                            this.set_value(
                                                                secs_to_slider(
                                                                    state.position,
                                                                    meta.duration,
                                                                ),
                                                                cx,
                                                            );
                                                        }
                                                        cx.notify();
                                                    },
                                                );
                                            })
                                        })
                                    }
                                    cx.notify();
                                }
                                AudioEvent::TrackLoaded(path) => {
                                    let meta = Metadata::read(path.clone()).expect("No metadata");
                                    cx.global_mut::<Controller>().set_meta_in_engine(meta);
                                    cx.notify();
                                }
                                _ => (),
                            },
                        )
                        .detach();

                        Root::new(view, window, cx)
                    })
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}

fn secs_to_slider(pos: u64, duration: u64) -> f32 {
    if duration == 0 {
        0.0
    } else {
        (pos as f32 / duration as f32) * 100.0
    }
}
