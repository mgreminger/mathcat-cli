use std::env;
use std::io::{self, Read};
use std::process;
use libmathcat::{set_rules_dir, set_mathml, get_spoken_text, set_preference};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("--version")) || args.contains(&String::from("-V")) {
        println!("mathcat-cli {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let mut rules_dir = String::new();
    let mut speech_style = String::from("ClearSpeak"); // Default
    let mut verbosity = String::from("Medium");        // Default

    // Parse CLI arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--rules-dir" if i + 1 < args.len() => {
                rules_dir = args[i + 1].clone();
                i += 1;
            }
            "--style" if i + 1 < args.len() => {
                let val = args[i + 1].clone();
                if !["ClearSpeak", "MathSpeak", "SimpleSpeak"].contains(&val.as_str()) {
                    eprintln!("Error: Invalid style '{}'. Valid options are: ClearSpeak, MathSpeak, SimpleSpeak.", val);
                    process::exit(1);
                }
                speech_style = val;
                i += 1;
            }
            "--verbosity" if i + 1 < args.len() => {
                let val = args[i + 1].clone();
                if !["Terse", "Medium", "Verbose"].contains(&val.as_str()) {
                    eprintln!("Error: Invalid verbosity '{}'. Valid options are: Terse, Medium, Verbose.", val);
                    process::exit(1);
                }
                verbosity = val;
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    // Fallbacks for rules_dir (Environment Var -> Executable Dir)
    if rules_dir.is_empty() {
        if let Ok(env_dir) = env::var("MATHCAT_RULES_DIR") {
            rules_dir = env_dir;
        }
    }
    if rules_dir.is_empty() {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let default_rules = exe_dir.join("Rules");
                if default_rules.exists() {
                    if let Some(rules_str) = default_rules.to_str() {
                        rules_dir = rules_str.to_string();
                    }
                }
            }
        }
    }

    if rules_dir.is_empty() {
        eprintln!("Error: MathCAT Rules directory not found.");
        process::exit(1);
    }

    // Initialize MathCAT
    if let Err(e) = set_rules_dir(rules_dir) {
        eprintln!("Failed to load MathCAT rules: {:?}", e);
        process::exit(1);
    }

    // Set Preferences (Style and Verbosity)
    if let Err(e) = set_preference("SpeechStyle".to_string(), speech_style) {
        eprintln!("Warning: Failed to set SpeechStyle: {:?}", e);
    }
    if let Err(e) = set_preference("Verbosity".to_string(), verbosity) {
        eprintln!("Warning: Failed to set Verbosity: {:?}", e);
    }

    // Process MathML
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        if set_mathml(input).is_ok() {
            if let Ok(speech) = get_spoken_text() {
                print!("{}", speech);
                return;
            }
        }
    }
    
    print!(""); // Fallback
}