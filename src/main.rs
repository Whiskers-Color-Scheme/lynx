use args_handler::handle_args;
use menu::show_main_menu;

mod args;
mod args_handler;
mod features;
mod menu;

fn main() {
    handle_args();
    show_main_menu();
}
