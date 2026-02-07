use gpui::{App, RenderImage};
use std::sync::Arc;
pub fn drop_image_from_app(cx: &mut App, image: Arc<RenderImage>) {
    for window in cx.windows() {
        let image = image.clone();

        window
            .update(cx, move |_, window, _| {
                window.drop_image(image).expect("Could not drop image");
            })
            .expect("Couldn't get window");
    }
}
