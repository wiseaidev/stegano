use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    about = "\x1b[38;5;214mThe ultimate steganography swiss knife army tool.\x1b[0m",
    name = "stegano",
    styles = styles()
)]
pub struct Cli {
    /// Sets the image input file.
    #[arg(short = 'i', long = "input")]
    pub input: String,

    /// Sets the output file.
    #[arg(short = 'o', long = "output", default_value_t = String::from("output.png"))]
    pub output: String,

    /// Enables metadata extraction.
    #[arg(short = 'm', long = "meta", default_value_t = false)]
    pub meta: bool,

    /// Read number of chunks.
    #[arg(short = 'n', long = "nb-chunks", default_value_t = 10)]
    pub nb_chunks: usize,

    /// The index of the start chunk to read from.
    #[arg(short = 'c', long = "start", default_value_t = 1)]
    pub start_chunk: usize,

    /// The index of the end chunk to stop reading at.
    #[arg(short = 'u', long = "end", default_value_t = 11)]
    pub end_chunk: usize,

    /// Suppresses output messages.
    #[arg(short = 's', long = "suppress", default_value_t = false)]
    pub suppress: bool,

    /// Sets the offset.
    #[arg(short = 'f', long = "offset", default_value_t = 10)]
    pub offset: usize,

    /// Enables injection.
    #[arg(short = 'j', long = "inject", default_value_t = false)]
    pub inject: bool,

    /// Sets the payload.
    #[arg(short = 'p', long = "payload", default_value_t = String::from("hello"))]
    pub payload: String,

    /// Sets the type.
    #[arg(short = 't', long = "type", default_value_t = String::from("PNG"))]
    pub r#type: String,

    /// Enables encoding.
    #[arg(short = 'e', long = "encode", default_value_t = false)]
    pub encode: bool,

    /// Enables decoding.
    #[arg(short = 'd', long = "decode", default_value_t = false)]
    pub decode: bool,

    /// Sets the key for payload encryption.
    #[arg(short = 'k', long = "key", default_value_t = String::from("key"))]
    pub key: String,
}
