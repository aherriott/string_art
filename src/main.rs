extern crate getopts;
use getopts::Options;
use std::env;

mod processor;
mod options;
use crate::processor::Processor;
use crate::options::CmdOptions;


static DEFAULT_INPUT_IMG : &str = "Data\\Images\\test_output.png";
static DEFAULT_OUTPUT_IMG : &str = "Data\\Images\\test_output.png";
static DEFAULT_INPUT_INSTR : &str = "Data\\Instructions\\default_instructions.csv";
static DEFAULT_OUTPUT_INSTR : &str = "Data\\Instructions\\output_instructions.csv";
static DEFAULT_SETTINGS : &str = "Data\\Settings\\default_settings.json";

fn check_option(option: Option<String>, default: &str) -> String {
    match option {
        Some(x) => x,
        None => String::from(default),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("n", "no_calc", "If set, program will use provided instructions rather than calculating new ones");
    opts.optopt("im", "input_img", "set input image filename", "NAME");
    opts.optopt("om", "output_img", "set output image filename", "NAME");
    opts.optopt("in", "input_instructions", "set input instructions filename", "NAME");
    opts.optopt("on", "output_instructions", "set output instructions filename", "NAME");
    opts.optopt("on", "settings", "settings filename", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    };

    let options = CmdOptions {
        calc_strings: {
            matches.opt_present("n")
        },
        input_img: {
            String::from(check_option(matches.opt_str("im"), DEFAULT_INPUT_IMG))
        },
        output_img: {
            String::from(check_option(matches.opt_str("om"), DEFAULT_OUTPUT_IMG))
        },
        input_instructions: {
            String::from(check_option(matches.opt_str("in"), DEFAULT_INPUT_INSTR))
        },
        output_instructions: {
            String::from(check_option(matches.opt_str("on"), DEFAULT_OUTPUT_INSTR))
        },
        settings: {
            String::from(check_option(matches.opt_str("s"), DEFAULT_SETTINGS))
        },
    };

    let processor = Processor::new(options);
    processor.process_image();
    
}
