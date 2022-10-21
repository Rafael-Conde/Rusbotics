// ui.label(format!("{:.1}", ctx.input().time));
#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// use robotics_program::gui_functions::get_gui;
use robotics_program::script;


fn main() 
{
	// let gui = get_gui();
	// gui.run_gui();
	script::symbolic_calculation();
}
