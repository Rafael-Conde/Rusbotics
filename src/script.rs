#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use pyo3::{prelude::*, types::PyDict};

pub fn symbolic_calculation() -> Result<(), PyErr>
{
	// add code to read the file instead of including the file String directly
	let test_run = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
	let script_library =
		include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/library.py"));
	unsafe {
		// trocar o caminho por um caminho relativo ao executável
		// Também implementar uma variável de retorno melhor para a função, 
		// quando ela for de fato gerar a imagem da equação 
		pyo3::with_embedded_python_interpreter(|py| -> Result<(), PyErr> {
			let globals = PyDict::new(py);
			let locals = PyDict::new(py);
			PyModule::from_code(py, script_library, "", "library")?;
			// let base_module = PyModule::from_code(py, py_app, "/python_app/app.py", "app").unwrap();
			// let app: Py<PyAny> = base_module.getattr("get_string").unwrap().into();
			// println!("Result: {}",app.call0(py).unwrap());
			py.run(test_run, Some(globals), Some(locals))
		})
	}
}
