use gio::prelude::*;

use soup::Session;
use soup::traits::*;

fn main() -> Result<(), glib::Error> {
    let session = Session::new();
    let input = session
        .request_http("GET", "http://example.com")?
        .send(Option::<&gio::Cancellable>::None)?;

    let output = gio::WriteOutputStream::new(std::io::stdout());

    output.splice(
        &input,
        gio::OutputStreamSpliceFlags::CLOSE_SOURCE,
        Option::<&gio::Cancellable>::None)?;

    Ok(())
}

