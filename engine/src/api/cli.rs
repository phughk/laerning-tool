use clap::Parser;

/// Laerning Tool CLI
#[derive(Parser, Debug)]
#[command(
    author = "Laerning Tool API", 
    version = "1.0", 
    about = "This is a software project to help people make flash cards to test themselves on things they want to learn." , 
    long_about = None
)]
pub struct ToolArgs {
    /// Socket address on which the server will listen.
    #[arg(short, long)]
    pub bind: Option<String>,

    /// SurrealDB Storage Location. Should be in URI format (ie file://)
    #[arg(long)]
    pub db_location: Option<String>,

    /// Dataset Directory location
    #[arg(long)]
    pub directory: Option<String>,
}
