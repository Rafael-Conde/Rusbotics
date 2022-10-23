// ui.label(format!("{:.1}", ctx.input().time));
#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(debug_assertions)]
macro_rules! debug_m {
    ($( $args:expr ),*) => { dbg!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! debug_m {
    ($( $args:expr ),*) => {};
}

use std::slice::Iter;

// use robotics_program::gui_functions::get_gui;
// use robotics_program::script;
use calamine::{open_workbook, DataType, Ods, Reader, Rows}; //, DataType};
use robotics_program::robotics::{Joint, JointType};

fn main()
{
    // let gui = get_gui();
    // gui.run_gui();
    // implement code to get the path from another place

    if let Ok(mut libreoffice) =
		open_workbook::<Ods<_>,_>("C:\\Rafael\\nvim_projects\\Rust\\robotics_program\\test_file.ods")
	{
		if let Some(Ok(r)) = libreoffice.worksheet_range_at(0)
		{
			let rows_count = r.rows().count();
			let mut joints: Vec<Box<dyn Joint>> = Vec::with_capacity(rows_count);
			// println!("initial vec capacity: {}", joints.capacity());
			debug_m!(joints.capacity());
			// let joints
			// Write code to dynamically allocate enough space for the whole rows
			// and then use this allocation to store the data read from the file
			let mut rows =  r.rows();
			match rows.next()
			{
				Some(&[DataType::String(ref a),DataType::String(ref rad_alpha),DataType::String(ref d),DataType::String(ref theta)]) => 
				{
					//Handle first line being the strings "a","rad_alpha","d","theta"
					if a != "a" || rad_alpha != "rad_alpha" || d != "d" || theta != "theta"
					{
						println!("When the first line is composed only of Strings, the Strings should be the following: \"a\",\"rad_alpha\",\"d\",\"theta\"");
						return;
					}
				},
				// for the following 2 other cases, use the funciont that's declared at the 
				// end of the file to process the data when the rows has the mix of Floats and
				// String
				Some(&[DataType::Float(a),DataType::Float(rad_alpha),DataType::Float(d),DataType::String(ref theta)]) => 
				{
					if theta == "X"
					{
						joints.push(Box::new(JointType::Rotational(a, rad_alpha, d)));
					}
				},
				Some(&[DataType::Float(a),DataType::Float(rad_alpha),DataType::String(ref d),DataType::Float(rad_theta)]) => 
				{
					// handles the first line being a prismatic joint
					if d == "X"
					{
						joints.push(Box::new(JointType::Prismatic(a, rad_alpha, rad_theta)));
					}
				},
				_ => println!("The first line doesn't matche the stablished pattern, checkout the default file to see a template"),
			}
		}
	}
    // if let Some(Ok(r)) = lib
}

// This is meant to be a private function to process the Rows<DataType> iterator
// so the processing logic is all in one place
// fn process_rows(iter: Rows<DataType>, vec_of_joints: &Vec<Box<dyn Joint>>);
