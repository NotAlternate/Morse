use rand::Rng;

pub enum Errors {
    NoArgument,
    NoInputFilename,
    NoOutputFilename,
    InvalidMode,
    InvalidFlag,
    DuplicateExplicit,
    BufferGetCharacter,
    TextEncodeOnly,
} pub fn parse(code: Errors) -> String {
    let symbol: &str = "\x1b[31;1mxx\x1b[0m";
    match code {
        Errors::NoArgument => format!("{} No arguments passed to the program.", symbol),
        Errors::NoInputFilename => format!("{} No input filename given for flag `-i` or `--input`", symbol),
        Errors::NoOutputFilename => format!("{} No output filename given for flag `-o` or `--output`", symbol),
        Errors::InvalidMode => format!("{} Invalid mode has been passed. `encode` or `decode` only (CASE-INSENSITIVE)", symbol),
        Errors::InvalidFlag => format!("{} Unknown flag has been passed.", symbol),
        Errors::BufferGetCharacter => format!("{} Unable to get character in Buffer", symbol),
        Errors::DuplicateExplicit => format!("{} Cannot select two explicitments, pick either `Text` or `Morse` mode", symbol),
        Errors::TextEncodeOnly => format!("{} The flag `-t` or `--text` can only be used in `encode` mode.", symbol),
    }
}

pub enum Commands {
    Help,
    HelpAlt,
} pub fn command(code: Commands) -> String {
    match code {
        Commands::Help => "\x1b[1m\x1b[32mMorse\x1b[0m v0.1.0\n\x1b[4mEncoder\x1b[0m and \x1b[4mdecoder\x1b[0m for morse code.\n\nProgram parameters:\n    \x1b[1mMorse\x1b[0m <ENCODE|DECODE> \"STRING\" [FLAGS]\n\nProgram flags:\n    \x1b[1m-o <FILE>\x1b[0m & \x1b[1m--output <FILE>\x1b[0m  ::  To output result to the file.\n    \x1b[1m-i <FILE>\x1b[0m & \x1b[1m--input <FILE>\x1b[0m   ::  To specify where to read from.\n    \x1b[1m-h\x1b[0m & \x1b[1m--help\x1b[0m   ::  Shows Morse's description and usage.\n    \x1b[1m-a\x1b[0m & \x1b[1m--audio\x1b[0m  ::  To output result as an audio (WAV).\n\n( ONLY WORKS IN ENCODE MODE: )\n    \x1b[1m-m\x1b[0m & \x1b[1m--morse\x1b[0m  ::  To explicitly specify input as Morse code.\n    \x1b[1m-t\x1b[0m & \x1b[1m--text\x1b[0m   ::  To explicitly specify input as Text.\n".to_string(),
        Commands::HelpAlt => "\x1b[1m\x1b[32m-- --- .-. ... .\x1b[0m v0.1.0\n\x1b[4mEncoder\x1b[0m and \x1b[4mdecoder\x1b[0m for morse code.\n\nProgram parameters:\n    \x1b[1mMorse\x1b[0m <ENCODE|DECODE> \"STRING\" [FLAGS]\n\nProgram flags:\n    \x1b[1m-o <FILE>\x1b[0m & \x1b[1m--output <FILE>\x1b[0m  ::  To output result to the file.\n    \x1b[1m-i <FILE>\x1b[0m & \x1b[1m--input <FILE>\x1b[0m   ::  To specify where to read from.\n    \x1b[1m-h\x1b[0m & \x1b[1m--help\x1b[0m   ::  Shows Morse's description and usage.\n    \x1b[1m-a\x1b[0m & \x1b[1m--audio\x1b[0m  ::  To output result as an audio (WAV).\n\n( ONLY WORKS IN ENCODE MODE: )\n    \x1b[1m-m\x1b[0m & \x1b[1m--morse\x1b[0m  ::  To explicitly specify input as Morse code.\n    \x1b[1m-t\x1b[0m & \x1b[1m--text\x1b[0m   ::  To explicitly specify input as Text.\n".to_string(),
    }
}

// wow you have found the code for an easter egg. congratulations
pub fn help() -> String {
    if rand::thread_rng().gen_range(0..2) == 1 { return command(Commands::HelpAlt) }
    command(Commands::Help)
}