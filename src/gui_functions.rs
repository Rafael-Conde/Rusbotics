// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
// Copyright (c) 2023 Rafael de Conde Reis. All rights reserved.

#![warn(clippy::all,
/*#![warn(*/clippy::pedantic,
		clippy::perf,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]


// #![allow(clippy::unwrap_used)]


use pyo3::{PyAny,Py};
use crate::{extract_robot_data_from_file, script};
use eframe::egui;
use rfd::MessageDialog;
use rfd::MessageLevel;
use egui_extras::image::RetainedImage;
use std::{sync::{Arc, Mutex, MutexGuard}, u8, path::Path, error::Error};

pub trait Gui
{
    fn run_gui(&self);
}

#[must_use]
pub fn get_gui() -> Box<dyn Gui>
{
	Box::<MyApp>::default()
}

impl Gui for MyApp
{
    #[allow(clippy::box_default)]
    fn run_gui(&self)
    {
        let options = eframe::NativeOptions::default();
        eframe::run_native("Robotics Program",
                           options,
                           Box::new(|_cc| Box::new(Self::default())));
    }
}

// #[derive(Default)]
struct MyApp
{
    comma_separated_data: String,
    picked_path: Option<Mutex<String>>,
    calculation_thread_state: Arc<Mutex<ThreadState>>,
    image_texture: Arc<Mutex<Option<RetainedImage>>>,
    missing_image_warned: bool,
    retained_image_zoom: f32,
}

impl Default for MyApp
{
	fn default() -> Self {
	    Self { comma_separated_data: Default::default(), picked_path: Default::default(), calculation_thread_state: Default::default(), image_texture: Default::default(), missing_image_warned: Default::default(), retained_image_zoom: 1f32 }
	}
}

#[derive(Default)]
enum ThreadState
{
    #[default]
    DidntRun,
    Running,
    Finished,
}

fn perform_calculations<P: AsRef<Path>>(path: P) -> Result<(Vec<u8>,String,Py<PyAny>), Box<dyn Error>>
{
    let joints = extract_robot_data_from_file(path)? // turn this unwrap
    												 // into a pop-up warning
                                                   .to_dh_table()
                                                   .get_joints();
    script::get_dh_matrix_image(&joints) 
}


fn button_generate_dh_matrix(picked_path: &MutexGuard<String>, calculation_thread_state: Arc<Mutex<ThreadState>>, image_texture: Arc<Mutex<Option<RetainedImage>>>)
{
    let temp = (*picked_path).clone();
    std::thread::spawn(move || {
		match perform_calculations(&temp)
		{
			Ok((image_bytes,_,_)) => {
        		// std::fs::write("test-page-0.png",image_bytes);
        		// let path = std::path::Path::new("test-page-0.png");
        		// if path.exists()
        		// {
				#[allow(clippy::option_if_let_else)]
            	// if let Ok(image_file_bytes) = std::fs::read(path)
            	// {
                match RetainedImage::from_image_bytes("equacao", &image_bytes)
                {
                    Ok(retained_image) => 
                    {
                        if let Ok(mut image_texture) = image_texture.lock()
                        {
                            *image_texture = Some(retained_image); 
                        };
                        // MessageDialog::new()
                        //     .set_level(MessageLevel::Info)
                        //     .set_title("Image displayed")
                        //     .set_description(&format!("Image displayed correctly!!"))
                        //     .show();
                    }
                    Err(err) =>
                    {
                        MessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Error displaying image")
                            .set_description(&format!("{err}"))
                            .show();
                    }
                }
            	// }
            	// else
            	// {
                //    	println!("error while reading the image from the file");
            	// }
            	if let Ok(mut state) = calculation_thread_state.lock()
            	{
                	*state = ThreadState::Finished;
            	};
        		// }
        	}
        	Err(err) =>
        	{
                MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Error performing calculations!")
                    .set_description(&format!("{err}"))
                    .set_buttons(rfd::MessageButtons::Ok)
                    .show();
            	println!("{err}");
            	if let Ok(mut state) = calculation_thread_state.lock()
            	{
                	*state = ThreadState::Finished;
            	};
        	}
        }
    });
    // *current_thread_state = ThreadState::Running;
}



impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file…").clicked()
            {
                if let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.picked_path = Some(Mutex::new(path.display().to_string()));
                }
            }
            ui.horizontal(|ui| {
                ui.label("Insert data here: ");
                ui.text_edit_singleline(&mut self.comma_separated_data);
            });
            if let Some(picked_path) = &self.picked_path
            {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    if let Ok(picked_path) = picked_path.lock()
                    {
                        ui.monospace(&(*picked_path));
                    }
                });
                if let Ok(mut current_thread_state) = self.calculation_thread_state.lock()
                {
					#[allow(clippy::significant_drop_in_scrutinee)]
                    match *current_thread_state
                    {
                        ThreadState::DidntRun =>
                        {
                            if ui.button("Generate equation").clicked()
                            {
                                if let Ok(picked_path) = picked_path.lock()
                                {
                                    let calculation_thread_state =
                                        Arc::clone(&self.calculation_thread_state);
                                    let image_texture = Arc::clone(&self.image_texture);
                                    button_generate_dh_matrix(&picked_path, calculation_thread_state, image_texture);
                                }
                                *current_thread_state = ThreadState::Running;
                            }
                        }
                        ThreadState::Running =>
                        {
                            ui.horizontal(|ui|
                                          {
                                              ui.add_enabled(false, egui::Button::new("Running calculations..."));
                                              ui.spinner();
                                          });
                            self.missing_image_warned = false;
                        }
                        ThreadState::Finished =>
                        {
                            if ui.button("Rerun calculations").clicked()
                            {
                                if let Ok(picked_path) = picked_path.lock()
                                {
                                    let calculation_thread_state =
                                        Arc::clone(&self.calculation_thread_state);
                                    let image_texture = Arc::clone(&self.image_texture);
                                    button_generate_dh_matrix(&picked_path, calculation_thread_state, image_texture);
                                }
                                *current_thread_state = ThreadState::Running;
                            }
                            if let Ok(image_texture) = self.image_texture.lock()
                            {
								#[allow(clippy::significant_drop_in_scrutinee)]
                                match *(image_texture)
                                {
                                    Some(ref image) => 
                                    {
                                        egui::ScrollArea::both().show(ui, |ui|{
                                            
                                            if image.show_scaled(ui, self.retained_image_zoom).hovered()
                                            {
												match ui.input().zoom_delta()
                                            	{
                                            		1f32 => (),
                                            		zoom if zoom < 1f32 => 
                                            		{
                                            			if (self.retained_image_zoom - (1f32 - zoom)) < 0.2f32
                                            			{
                                            				self.retained_image_zoom = 0.2f32;
                                            			}
                                            			else
                                            			{
                                            				self.retained_image_zoom += - (1f32 - zoom);
                                            			}
                                            		},
                                            		zoom if zoom > 1f32 => 
                                            		{
                                            			if (self.retained_image_zoom + (zoom - 1f32)) > 5f32
                                            			{
                                            				self.retained_image_zoom = 5f32;
                                            			}
                                            			else
                                            			{
                                            				self.retained_image_zoom += zoom - 1f32;
                                            			}
                                            		},
                                            		_ => unreachable!()
                                            	}
                                            }
                                        });
                                    },
                                    None if !self.missing_image_warned  => 
                                    {
                                        self.missing_image_warned = true;
                                        println!("It's believed that there should be an image to be displayed");
                                    },
                                    None => ()
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}
