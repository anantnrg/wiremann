use crate::errors::ScannerError;
use std::fs::File;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

fn open_stream(path: &Path) -> Result<MediaSourceStream, ScannerError> {
    let mut hint = Hint::new();

    let source = {
        if let Some(ext) = path.extension().and_then(|this| this.to_str()) {
            hint.with_extension(ext);
        }

        Box::new(File::open(path)?)
    };

    let mss = MediaSourceStream::new(source, Default::default());

    let mut probe = symphonia::default::get_probe().format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())?;
}