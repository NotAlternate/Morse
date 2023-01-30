use std::process::exit;
use morse::strings;
use std::env;

#[allow(unused_assignments)]
#[allow(non_snake_case)]
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 { eprintln!("{}\n\n{}", strings::parse(strings::Errors::NoArgument), strings::help()); exit(-1); }

    // ENCODE OR DECODE FLAG
    let mut isEncode = false;
    let mut isDecode = false;
    let mode = args[1].to_lowercase();

    if mode == "encode" || mode == "en" { isEncode = true; }
    else if mode == "decode" || mode == "de" { isDecode = true; }
    else { eprintln!("{}", strings::parse(strings::Errors::InvalidMode)); exit(-1); }

    println!("E: {}\nD: {}", isEncode, isDecode); // debug

    // FLAGS' VARIABLES
    let mut isAudio = false;
    let mut isMorse = false;
    let mut isText  = false;

    // PARSING FOR FLAGS
    let mut index: usize = 3; // Skipping 3 arguments

    while index < args.len() {
        let value = args[index].to_lowercase().to_string();
        if value == "-h" || value == "--help" { println!("{}", strings::command(strings::Commands::Help)); exit(0); }

        else if value == "-a" || value == "--audio" { isAudio = true }
        else if value == "-m" || value == "--morse" { if isText == true { eprintln!("{}", strings::parse(strings::Errors::DuplicateExplicit)); exit(-1); } isMorse = true }
        else if value == "-t" || value == "--text"  {
            if isDecode == true { eprintln!("{}", strings::parse(strings::Errors::TextEncodeOnly)); exit(-1); }
            if isMorse == true { eprintln!("{}", strings::parse(strings::Errors::DuplicateExplicit)); exit(-1); }
            isText = true
        }

        else { eprintln!("{}", strings::parse(strings::Errors::InvalidFlag)); exit(-1); }
        index += 1;
    }

    println!("I: {}\nFlags:\nA: {}\nM: {}\nT: {}", args[2], isAudio, isMorse, isText) // debug
}