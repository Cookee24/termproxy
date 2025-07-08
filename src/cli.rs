use clap::Parser;

use crate::utils::Terminal;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Init proxy environment variables with specified terminal
    Init {
        terminal: Terminal,

        #[clap(flatten)]
        options: Box<Options>,
    },

    /// Get current terminal's proxy environment variables
    Cat,
}

#[derive(Parser, Debug)]
pub struct Options {
    /// Output to file
    #[arg(short, long)]
    pub output: Option<String>,

    #[clap(flatten)]
    pub query: QueryOptions,

    #[clap(flatten)]
    pub r#override: OverrideOptions,
}

#[derive(Parser, Debug)]
pub struct QueryOptions {
    /// Address for query http proxy
    #[arg(long, default_value = "http://google.com")]
    pub http_query_addr: String,

    /// Address for query https proxy
    #[arg(long, default_value = "https://google.com")]
    pub https_query_addr: String,

    /// Address for query ftp proxy
    #[arg(long, default_value = "ftp://google.com")]
    pub ftp_query_addr: String,

    /// Address for query all proxy
    #[arg(long, default_value = "tcp://google.com")]
    pub all_query_addr: String,

    /// Addresses for query no proxy, separated by comma
    #[arg(long, default_value = "localhost", value_delimiter = ',')]
    pub no_query_addrs: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct OverrideOptions {
    /// Override http proxy fetched from system
    #[arg(long)]
    pub http_proxy: Option<String>,

    /// Override https proxy fetched from system
    #[arg(long)]
    pub https_proxy: Option<String>,

    /// Override ftp proxy fetched from system
    #[arg(long)]
    pub ftp_proxy: Option<String>,

    /// Override all proxy fetched from system
    #[arg(long)]
    pub all_proxy: Option<String>,

    /// Override no proxy fetched from system
    #[arg(long)]
    pub no_proxy: Option<String>,

    /// Do not detect proxy from system
    #[arg(long)]
    pub no_detect: bool,

    /// Force SOCKS5H protocol for bare IP addresses on Windows
    #[arg(long)]
    pub force_socks5h: bool,
}
