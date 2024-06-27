use std::path::PathBuf;

use clap::{Parser, Subcommand, Args, ArgAction};
use log::LevelFilter;

type BoxedError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;
type UnitResult<'a> = Result<(), BoxedError<'a>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true, propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: Verbosity
}

#[derive(Args)]
#[group(multiple = false)]
pub struct Verbosity {
    #[arg(short = 'd', long = "debug", help = "Enable debugging output", global = true)]
    pub debug: bool,

    #[arg(short = 'v', long = "verbose", help = "Enable verbose output", global = true)]
    pub verbose: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress informational messages", global = true)]
    pub quiet: bool
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new secret key or username
    Generate {
        /// The sub-command to execute
        #[command(subcommand)]
        command: GenerateCommands
    },
    /// Cryptographically analyze a piece of data
    Analyze {
        /// A path to a file on a filesystem, or leave empty to read from STDIN
        input: Option<PathBuf>
    },
    /// Create a visualization of an arbitary piece of data.
    Visualize {
        #[arg(help = "A path to a file on a filesystem, or leave empty to read from STDIN")]
        input: Option<PathBuf>,

        #[arg(short = 'o', long = "output", help = "A path on a filesystem where data should be written, or leave empty to write to STDOUT")]
        output: Option<PathBuf>
    }
}

#[derive(Subcommand)]
pub enum GenerateCommands {
    /// Generate random bytes
    Bytes {
        /// The number of bytes to generate
        length: usize
    },
    /// Generate random bytes and encode them as a hexadecimal string
    Hex {
        #[arg(short = 'u', long = "uppercase", help = "Print hexadecimal digits in uppercase")]
        uppercase: bool,

        /// The number of bytes to generate
        length: usize
    },
    /// Generate random bytes and encode them as a Base64 string
    Base64 {
        #[arg(short = 'u', long = "url-safe", help = "Use a URL-safe alphabet")]
        url_safe: bool,

        /// The number of bytes to generate
        length: usize
    },
    /// Generate a random password with a configurable character set
    Password {
        #[arg(short = 'D', long = "no-digits", help = "Don't include any digits", action = ArgAction::SetFalse)]
        numbers: bool,

        #[arg(short = 'S', long = "no-symbols", help = "Don't include any symbols", action = ArgAction::SetFalse)]
        symbols: bool,

        /// The number of characters to generate
        length: usize,

        /// How many passwords to generate
        count: Option<usize>
    },
    /// Generate a passphrase composed of words chosen at random from a wordlist
    Passphrase {
        #[arg(short = 'p', long = "path", help = "The wordlist file to read into memory")]
        path: Option<PathBuf>,

        #[arg(short = 'D', long = "delimiter", help = "The string used to separate words from each other in the wordlist", default_value = "\n")]
        delimiter: String,

        #[arg(short = 's', long = "separator", help = "A string used to separate words in the passphrase", default_value = " ")]
        separator: String,

        /// The number of words to generate
        length: usize,

        /// How many passphrases to generate
        count: Option<usize>
    },
    /// Generate a random pronounceable username
    Username {
        #[arg(short = 'C', long = "capitalize", help = "Make the first letter uppercase", global = true)]
        capitalize: bool,

        #[command(subcommand)]
        command: UsernameCommands
    },
    /// Generate a random sequence of digits
    Digits {
        /// The number of digits to generate
        length: usize,

        /// How many sequences of digits to generate
        count: Option<usize>
    },
    /// Generate a random number
    Number {
        /// The smallest number that can be generated
        minimum: usize,

        /// The largest number that can be generated
        maximum: usize,

        /// How many numbers to generate
        count: Option<usize>
    },
    /// Generate a random word using a Markov model
    Markov {
        #[arg(short = 'C', long = "no-capitalize", help = "Do not capitalize words", action = ArgAction::SetFalse)]
        capitalize: bool,

        #[arg(short = 'i', long = "input", help = "A path to a file containing a corpus to use")]
        path: Option<PathBuf>,

        #[command(flatten)]
        length_range: LengthRange,

        #[command(flatten)]
        model_parameters: ModelParameters,

        #[command(flatten)]
        cache_control: CacheControl,

        /// How many words to generate
        count: Option<usize>
    },
}

#[derive(Subcommand)]
pub enum UsernameCommands {
    /// Generate a simple pronounceable username that alternates between vowels and consonants
    Simple {
        /// The number of characters to generate
        length: usize,

        /// How many simple usernames to generate
        count: Option<usize>
    },
    /// Generate a complex pronounceable username that is constructed from syllables
    Complex {
        /// The number of syllables to generate
        length: usize,

        /// How many syllabic usernames to generate
        count: Option<usize>
    }
}

#[derive(Args)]
#[group(multiple = true)]
pub struct LengthRange {
    #[arg(short = 'm', long = "min", help = "The minimum length of the word", default_value = "2")]
    pub minimum: usize,

    #[arg(short = 'M', long = "max", help = "The maximum length of the word", default_value = "10")]
    pub maximum: usize
}

#[derive(Args)]
#[group(multiple = true)]
pub struct ModelParameters {
    #[arg(short = 'o', long = "order", help = "The model ordering to use", default_value = "3")]
    pub order: usize,

    #[arg(short = 'p', long = "prior", help = "The Dirichlet probability of picking any random letter", default_value = "0.0")]
    pub prior: f64,

    #[arg(short = 'b', long = "backoff", help = "Use Katz back-off model")]
    pub backoff: bool
}

#[derive(Args)]
#[group(multiple = false)]
pub struct CacheControl {
    #[arg(short = 'N', long = "no-cache", help = "Do not use a cached model")]
    pub no_cache: bool,

    #[arg(short = 'R', long = "rebuild-cache", help = "Rebuild the cached model")]
    pub rebuild_cache: bool
}

impl Verbosity {
    fn to_filter(&self) -> LevelFilter {
        if self.debug { LevelFilter::Trace }
        else if self.verbose { LevelFilter::Debug }
        else if self.quiet { LevelFilter::Warn }
        else { LevelFilter::Info }
    }
}

pub fn parse() -> Arguments {
    Arguments::parse()
}

pub fn setup_logging<'a>(verbosity: &Verbosity) -> UnitResult<'a> {
    let filter = verbosity.to_filter();

    env_logger::builder()
        .filter_level(filter)
        .format_level(true)
        .format_target(false)
        .format_module_path(false)
        .format_timestamp_secs()
        .format_indent(None)
        .parse_default_env()
        .try_init()?;

    Ok(())
}
