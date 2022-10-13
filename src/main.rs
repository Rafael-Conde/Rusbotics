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
use pyo3::prelude::*;

fn main()
{
	// let gui = get_gui();
	// gui.run_gui();
	unsafe {
		// trocar o caminho por um caminho relativo ao executável
		let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
		// println!("Code:\n{}",py_app);
		let result = pyo3::with_embedded_python_interpreter(|py|  { 
			let base_module = PyModule::from_code(py, py_app, "/python_app/app.py", "app").unwrap();
			let app: Py<PyAny> = base_module.getattr("get_string").unwrap().into();
			println!("Result: {}",app.call0(py).unwrap());
		});
	}
}
