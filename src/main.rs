mod command_lex;
mod run;
mod user_handling;
mod commands;
mod utils;
mod util_helpers;

use command_lex::command_lex::*;
use run::run;

fn main() {
    run();
}
