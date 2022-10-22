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
// use robotics_program::script;
use calamine::{open_workbook, Ods, Reader, DataType};//, DataType};

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
				Some(&[DataType::Float(ref a),DataType::Float(ref rad_alpha),DataType::Float(ref d),DataType::String(ref theta)]) => 
				{
					// handles the first line being rotational joint
					if theta == "X"
					{
						
					}
				},
				Some(&[DataType::Float(ref a),DataType::Float(ref rad_alpha),DataType::String(ref d),DataType::Float(ref theta)]) => 
				{
					// handles the first line being a prismatic joint
				},
				_ => println!("The first line doesn't matche the stablished pattern, chekout the default file to see a template"),
			}
			// match rows.next() 
			// {
				// Some(&[calamine::Datatype::String(ref a),calamine::Datatype::String(ref rad_alpha),calamine::Datatype::String(ref d),calamine::Datatype::String(ref theta)]) => 
				// {
				// 	if a != "a" || rad_alpha != "rad_alpha" || d != "d" || theta != "theta"
				// 	{
				// 		println!("When the first line is composed only of Strings, the Strings should be the following: \"a\",\"rad_alpha\",\"d\",\"theta\"");
				// 		return;
				// 	}
				// },
				// // implementar uma função para receber a linha e apartir dela devolver uma
				// // variáveldo tipo junta
				// Some(&[Datatype::Float(a),Datatype::Float(rad_alpha),Datatype::Float(d),Datatype::String(ref theta)]) => 
				// {
				// 	if theta == "X"
				// 	{
				// 		// write code to store the data read into a dynamically allocated 
				// 		// space
				// 	}
				// },
			// 	_ => (),
			// }
		}
	}
	// if let Some(Ok(r)) = lib
}
