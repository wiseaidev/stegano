use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};

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
    propagate_version = true,
    styles = styles()
)]
pub struct Cli {
    /// Subcommands for encryption and decryption.
    #[command(subcommand)]
    pub command: Option<SteganoCommands>,
}

/// Represents available subcommands for the stegano CLI.
#[derive(Subcommand, Debug)]
pub enum SteganoCommands {
    /// Subcommand for encryption.
    Encrypt(EncryptCmd),

    /// Subcommand for decryption.
    Decrypt(DecryptCmd),

    /// Subcommand for showing metadata.
    ShowMeta(ShowMetaCmd),
}

/// Subcommand for encryption.
#[derive(Parser, Debug)]
pub struct EncryptCmd {
    /// Sets the input file for injecting the payload.
    #[arg(short = 'i', long = "input")]
    pub input: String,

    /// Sets the output file for generating a new file with the injected payload.
    #[arg(short = 'o', long = "output", default_value_t = String::from("output.png"))]
    pub output: String,

    /// Sets the key for payload encryption.
    #[arg(short = 'k', long = "key", default_value_t = String::from("key"))]
    pub key: String,

    /// Suppresses output messages.
    #[arg(short = 's', long = "suppress", default_value_t = false)]
    pub suppress: bool,

    /// Sets the offset.
    #[arg(short = 'f', long = "offset", default_value_t = 10)]
    pub offset: usize,

    /// Sets the payload.
    #[arg(short = 'p', long = "payload", default_value_t = String::from("hello"))]
    pub payload: String,

    /// Sets the type.
    #[arg(short = 't', long = "type", default_value_t = String::from("PNG"))]
    pub r#type: String,

    /// Sets the algorithm.
    #[arg(short = 'a', long = "algo", default_value_t = String::from("aes"))]
    pub algorithm: String,
}

/// Subcommand for decryption.
#[derive(Parser, Debug)]
pub struct DecryptCmd {
    /// Sets the input file for decrypting and extracting the payload.
    #[arg(short = 'i', long = "input")]
    pub input: String,

    /// Sets the output file for generating a new file with no payload, aka restoring the original file.
    #[arg(short = 'o', long = "output", default_value_t = String::from("output.png"))]
    pub output: String,

    /// Sets the key for payload encryption.
    #[arg(short = 'k', long = "key", default_value_t = String::from("key"))]
    pub key: String,

    /// Suppresses output messages.
    #[arg(short = 's', long = "suppress", default_value_t = false)]
    pub suppress: bool,

    /// Sets the offset.
    #[arg(short = 'f', long = "offset", default_value_t = 10)]
    pub offset: usize,

    /// Sets the payload.
    #[arg(short = 'p', long = "payload", default_value_t = String::from("hello"))]
    pub payload: String,

    /// Sets the type.
    #[arg(short = 't', long = "type", default_value_t = String::from("PNG"))]
    pub r#type: String,

    /// Sets the algorithm.
    #[arg(short = 'a', long = "algo", default_value_t = String::from("aes"))]
    pub algorithm: String,
}

/// Subcommand for showing metadata.
#[derive(Parser, Debug)]
pub struct ShowMetaCmd {
    /// Sets the image input file.
    #[arg(short = 'i', long = "input")]
    pub input: String,

    /// Read number of chunks.
    #[arg(short = 'n', long = "nb-chunks", default_value_t = 100)]
    pub nb_chunks: usize,

    /// The index of the start chunk to read from.
    #[arg(short = 's', long = "start", default_value_t = 0)]
    pub start_chunk: usize,

    /// The index of the end chunk to stop reading at.
    #[arg(short = 'e', long = "end", default_value_t = 100)]
    pub end_chunk: usize,

    /// Suppresses output messages.
    #[arg(short = 'r', long = "suppress", default_value_t = false)]
    pub suppress: bool,

    /// Sets the type.
    #[arg(short = 't', long = "type", default_value_t = String::from("PNG"))]
    pub r#type: String,
}
