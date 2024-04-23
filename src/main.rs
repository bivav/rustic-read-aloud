use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

use std::{fs::File, io::Write};

use anyhow::Result;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://api.streamelements.com/kappa/v2/speech";

    let text = "Any text here";

    let params = [("voice", "Brian"), ("text", text.trim())];

    let client = Client::new();
    let response = client.get(url).query(&params).send().await?;

    if response.status().is_success() {
        let mut file = File::create("output.mp3")?;
        let bytes = response.bytes().await?;

        file.write_all(&bytes)?;

        let cursor = Cursor::new(bytes);

        // Setting up audio output stream
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        // Try to decode the mp3 from bytes cursor
        let decoder = Decoder::new_mp3(cursor)?;
        sink.append(decoder);

        sink.sleep_until_end();

        println!("Request was successful");
    } else {
        println!(
            "Request failed with status: {}, {:?}",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}
