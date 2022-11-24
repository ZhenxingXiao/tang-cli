use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Tang-CLI Command
pub struct CliCommand{
    /// time in ms between two ticks.
    #[argh(option, default = "100")]
    pub tick_rate: u64,

    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    pub enhanced_graphics: bool,

    #[argh(subcommand)]
    pub nested: CliSubCommandEnum
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CliSubCommandEnum{
    Performance(PerformanceCommand),
    Crypto(CryptoCommand),
    Info(InfoCommand)
}

#[derive(FromArgs, PartialEq, Debug)]
/// performance subcommand.
#[argh(subcommand, name = "performance")]
pub struct PerformanceCommand{
    #[argh(switch, short='g')]
    /// whether include graphic
    pub graphic: bool,

    #[argh(switch, short='i')]
    /// whether include internet
    pub internet: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// crypto subcommand.
#[argh(subcommand, name = "crypto")]
pub struct CryptoCommand{
    #[argh(option, default = "String::from(\"AES256\")")]
    /// the algorimth of cryptography
    pub algorithm: String
}

#[derive(FromArgs, PartialEq, Debug)]
/// information subcommand.
#[argh(subcommand, name = "info")]
pub struct InfoCommand{

}