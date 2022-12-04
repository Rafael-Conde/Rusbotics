#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use pyo3::{prelude::*, types::PyDict};

use crate::robotics::{Errors, Joint, JointType};
use std::fs::OpenOptions;
use std::io::Write;
use std::{env, error::Error};

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
    // env::set_var("FONTCONFIG_PATH", env::current_dir().unwrap());
    let input = joints_to_python_code_for_method_input(joints).unwrap();
    let test_run = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/app.py"));
    let script_library =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/python_app/library.py"));
    let mut tex_code = Default::default();
    unsafe {
        pyo3::with_embedded_python_interpreter(|py| -> Result<(), PyErr> {
            let globals = PyDict::new(py);
            let locals = PyDict::new(py);
            PyModule::from_code(py, script_library, "", "library")?;
            py.run(&input, Some(globals), Some(locals))?;
            py.run(test_run, Some(globals), Some(locals)).unwrap();
            let latex_equation: &str = locals.get_item("latex_equation").unwrap().extract()?;
            let mut latex_equation = latex_equation.to_string();
            tex_code = format!(
                               "
\\documentclass{{standalone}}
\\usepackage{{amsmath}}
\\begin{{document}}
	\\( \\displaystyle {} \\)
\\end{{document}}
",
                               latex_equation
            );
            Ok(())
        });
    }

    println!("The text is: \n{}", tex_code);
    let mut resp: Vec<u8> = tectonic::latex_to_pdf(tex_code).unwrap();
    let mut file = OpenOptions::new().write(true)
                                     .truncate(true)
                                     .create(true)
                                     .open("test.pdf")?;

    file.write_all(&mut resp);
    Ok(())
}
