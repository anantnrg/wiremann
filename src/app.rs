use std::thread;

use crate::audio::engine::Audio;
use crate::controller::Controller;
use crate::controller::state::AppState;
use crate::errors::AppError;
use crate::scanner::Scanner;
use crate::ui::assets::Assets;
use gpui::*;
use gpui_component::*;

use crate::ui::wiremann::Wiremann;

pub fn run() -> Result<(), AppError> {
    let assets = Assets {};
    let app = Application::new().with_assets(assets.clone());

    let (mut audio, audio_tx, audio_rx) = Audio::new();
    let (mut scanner, scanner_tx, scanner_rx) = Scanner::new();

    thread::spawn(move || {
        audio.run()
    });

    thread::spawn(move || {
        scanner.run()
    });

    app.run(move |cx| {
        gpui_component::init(cx);
        let bounds = Bounds::centered(None, size(px(1280.0), px(760.0)), cx);
        assets.load_fonts(cx).expect("Could not load fonts");

        let controller = Controller::new(cx.new(|_| AppState::default()), audio_tx, audio_rx, scanner_tx, scanner_rx);

        cx.set_global(controller.clone());

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

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, AppError>(())
        })
        .detach();
    });

    Ok(())
}
