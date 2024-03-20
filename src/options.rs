// static DEFAULT_INPUT_IMG : &str = "Data\\Images\\test_output.png";
// static DEFAULT_OUTPUT_IMG : &str = "Data\\Images\\test_output.png";
// static DEFAULT_INPUT_INSTR : &str = "Data\\Instructions\\default_instructions.csv";
// static DEFAULT_OUTPUT_INSTR : &str = "Data\\Instructions\\output_instructions.csv";
// static DEFAULT_SETTINGS : &str = "Data\\Settings\\default_settings.json";

pub struct CmdOptions {
    pub calc_strings: bool,
    pub input_img: String,
    pub output_img: String,
    pub input_instructions: String,
    pub output_instructions: String,
    pub settings: String,
}

// let SETTINGS_TAGS = ['img_width', 'num_pegs', 'max_color_threads', 'total_threads', 'thread_thickness', 'eval_thickness', 'neighbors_exclude', 'thread_alpha', 'ban_repeats', 'max_repeats', 'autogen_colors', 'num_colors', 'colors']
