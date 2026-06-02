pub struct Style;

impl Style {
    pub const RESET: &'static str = "\x1b[0m";

    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";

    pub const RESET_BOLD: &'static str = "\x1b[22m";
    pub const RESET_DIM: &'static str = "\x1b[22m";
    pub const RESET_ITALIC: &'static str = "\x1b[23m";
    pub const RESET_UNDERLINE: &'static str = "\x1b[24m";

    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";

    pub const BRIGHT_BLACK: &'static str = "\x1b[90m";
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";

    pub const BG_BLACK: &'static str = "\x1b[40m";
    pub const BG_RED: &'static str = "\x1b[41m";
    pub const BG_GREEN: &'static str = "\x1b[42m";
    pub const BG_YELLOW: &'static str = "\x1b[43m";
    pub const BG_BLUE: &'static str = "\x1b[44m";
    pub const BG_MAGENTA: &'static str = "\x1b[45m";
    pub const BG_CYAN: &'static str = "\x1b[46m";
    pub const BG_WHITE: &'static str = "\x1b[47m";

    pub const BG_BRIGHT_BLACK: &'static str = "\x1b[100m";
    pub const BG_BRIGHT_RED: &'static str = "\x1b[101m";
    pub const BG_BRIGHT_GREEN: &'static str = "\x1b[102m";
    pub const BG_BRIGHT_YELLOW: &'static str = "\x1b[103m";
    pub const BG_BRIGHT_BLUE: &'static str = "\x1b[104m";
    pub const BG_BRIGHT_MAGENTA: &'static str = "\x1b[105m";
    pub const BG_BRIGHT_CYAN: &'static str = "\x1b[106m";
    pub const BG_BRIGHT_WHITE: &'static str = "\x1b[107m";
}
