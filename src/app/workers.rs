use std::thread;

pub fn spawn_worker<F, E>(name: &'static str, f: F)
where
    F: FnOnce() -> Result<(), E> + Send + 'static,
    E: core::fmt::Debug,
{
    thread::spawn(move || {
        if let Err(e) = f() {
            eprintln!("{name} thread crashed with error: {e:?}");
        }
    });
}
