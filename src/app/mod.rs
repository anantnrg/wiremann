mod events;
mod paths;
mod window;
mod workers;

use crate::cacher::Cacher;
use crate::image_processor::ImageProcessor;
use crate::lyrics_manager::LyricsManager;
use crate::system_integration::SystemIntegration;
use crate::worker_config::{WorkerConfig, calculate_worker_config};
use crate::{
    audio::Audio,
    controller::{Controller, state::AppState},
    errors::AppError,
    scanner::Scanner,
    ui::{
        assets::Assets,
        res_handler::{Event, ResHandler},
        wiremann::Wiremann,
    },
};

use gpui::{AppContext, Application, Result};
use raw_window_handle::HasWindowHandle;
use std::sync::Arc;

use events::{spawn_event_loop, subscribe_controller_events};
use paths::{ensure_app_paths, get_app_paths};
use window::build_window_options;
use workers::spawn_worker;

static ICON_PNG: &[u8] = include_bytes!("../../assets/logos/logo.png");

pub fn run() -> Result<(), AppError> {
    let assets = Assets {};

    Application::new()
        .with_assets(assets.clone())
        .run(move |cx| {
            assets.load_fonts(cx).expect("Could not load fonts");

            let WorkerConfig {
                metadata: metadata_workers,
                thumbnail: thumbnail_workers,
                cacher: cacher_workers,
            } = calculate_worker_config();

            let app_paths = get_app_paths();

            ensure_app_paths(&app_paths);

            let app_icon = gpui::WindowIcon::from_png_bytes(ICON_PNG).ok();

            cx.open_window(build_window_options(app_icon, cx), |window, cx| {
                let (mut audio, audio_tx, audio_rx) = Audio::new();

                let (mut scanner, scanner_tx, scanner_rx) = Scanner::new(app_paths.clone());

                let (cacher, cacher_tx, cacher_rx) = Cacher::new(app_paths.clone());

                let (mut image_processor, image_processor_tx, image_processor_rx) =
                    ImageProcessor::new(app_paths.clone());

                let raw_window_handle = window.window_handle().ok().map(|this| this.as_raw());

                let (mut system_integration, system_integration_tx, system_integration_rx) =
                    SystemIntegration::new(raw_window_handle, app_paths);

                let (mut lyrics_manager, lyrics_manager_tx, lyrics_manager_rx) =
                    LyricsManager::new();

                let controller = Controller::new(
                    cx.new(|_| AppState::default()),
                    audio_tx,
                    audio_rx,
                    scanner_tx,
                    scanner_rx,
                    cacher_tx,
                    cacher_rx,
                    image_processor_tx,
                    image_processor_rx,
                    system_integration_tx,
                    system_integration_rx,
                    lyrics_manager_tx,
                    lyrics_manager_rx,
                );

                spawn_worker("audio", move || audio.run());

                spawn_worker("scanner", move || scanner.run(metadata_workers));

                spawn_worker("cacher", move || cacher.run(cacher_workers));

                spawn_worker("image processor", move || {
                    image_processor.run(thumbnail_workers)
                });

                spawn_worker("system integration", move || system_integration.run());

                spawn_worker("lyrics manager", move || lyrics_manager.run());

                cx.set_global(controller.clone());

                let view = cx.new(Wiremann::new);

                let res_handler = cx.new(|_| ResHandler {});
                let arc_res = Arc::new(res_handler.clone());

                spawn_event_loop(cx, controller.clone(), arc_res.clone());

                subscribe_controller_events(cx, &res_handler, controller.clone(), &view);

                view
            })
            .expect("Application panicked.");

            cx.activate(true);
        });

    Ok(())
}
