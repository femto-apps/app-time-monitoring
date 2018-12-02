extern crate winapi;

mod active_app;

fn main() {
	println!("Active Application Title: {:#?}", active_app::get_foreground_window_title());
	println!("Active Application Path: {:#?}", active_app::get_foreground_window_path());
	println!("Module Application Name: {:#?}", active_app::get_module_file_name_ex());
}