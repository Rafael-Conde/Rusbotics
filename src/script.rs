#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use pyo3::{
    prelude::*,
    types::{PyDict, PyString},
};

use crate::robotics::{Errors, Joint, JointType};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

// implement a function that converts a Vec<Box<dyn Joint>> into python code that
// can then be converted into the table that is used in the methods for symbolic
// calculatins
fn joints_to_python_code_for_method_input(joints: Vec<Box<dyn Joint>>)
                                          -> Result<String, Box<dyn Error>>
{
    if joints.is_empty()
    {
        return Err(Box::new(Errors::SimpleError("No Joint was provided")));
    }
    let mut python_code_input = String::from("tabela_DH = ([[");
    let mut tailing = "],[";
    let mut joint_it = joints.iter().peekable();
    let mut i = 0;
    while let Some(joint) = joint_it.next()
    {
        i += 1;
        if joint_it.peek().is_none()
        {
            tailing = "]])\n\n";
        }
        match joint.get_joint_type()
        {
            JointType::Prismatic(a, rad_alpha, theta) =>
            {
                python_code_input.push_str(&format!("'{rad_alpha}','{a}','d_{i}','{theta}'{tailing}"));
            }
            JointType::Rotational(a, rad_alpha, d) =>
            {
                python_code_input.push_str(&format!("'{rad_alpha}','{a}','{d}','theta_{i}'{tailing}"));
            }
        }
    }
    tailing = "','";
    python_code_input.push_str("joints = (['");
    let mut joint_it = joints.into_iter().peekable();
    while let Some(joint) = joint_it.next()
    {
        if joint_it.peek().is_none()
        {
            tailing = "'])\n\n";
        }
        match joint.get_joint_type()
        {
            JointType::Prismatic(_, _, _) =>
            {
                python_code_input.push('p');
                python_code_input.push_str(tailing);
            }
            JointType::Rotational(_, _, _) =>
            {
                python_code_input.push('r');
                python_code_input.push_str(tailing);
            }
        }
    }
    Ok(python_code_input)
}

pub fn get_matrix_image(joints: Vec<Box<dyn Joint>>) -> Result<(), PyErr>
{
    // add code to read the file instead of including the file String directly
    let input = joints_to_python_code_for_method_input(joints).unwrap();
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
            py.run(&input, Some(globals), Some(locals))?;
            // let base_module = PyModule::from_code(py, py_app, "/python_app/app.py", "app").unwrap();
            // let app: Py<PyAny> = base_module.getattr("get_string").unwrap().into();
            // println!("Result: {}",app.call0(py).unwrap());
            py.run(test_run, Some(globals), Some(locals)).unwrap();
            let latex_equation: &str = locals.get_item("latex_equation").unwrap().extract()?;
            println!("Just got the &str from the equation");
            let mut latex_equation = latex_equation.to_string();
            latex_equation = latex_equation.replace('(', "\\(");
            latex_equation = latex_equation.replace(')', "\\)");
            // latex_equation = latex_equation.replace('{', "\\{");
            // latex_equation = latex_equation.replace('}', "\\}");
            // latex_equation = latex_equation.replace('[', "\\[");
            // latex_equation = latex_equation.replace(']', "\\]");
            println!("The string that will be appended to the url:\n\n{}",
                     latex_equation);
            let mut string_file = OpenOptions::new().write(true)
                                                    // either use ? or unwrap since it returns a Result
                                                    .create(true)
                                                    .open("url.txt")?;
            string_file.write(latex_equation.as_bytes());
            let mut url = String::from("https://latex.codecogs.com/png.latex?\\bg_white&space;");
            url.push_str(&latex_equation);
            let mut resp: Vec<u8> = reqwest::blocking::get(&url).unwrap()
                                                                .bytes()
                                                                .unwrap()
                                                                .to_vec();
            let mut file = OpenOptions::new().write(true)
                                             // either use ? or unwrap since it returns a Result
                                             .create(true)
                                             .open("test.png")?;

            file.write_all(&mut resp);

            Ok(())
        })
    }
}
