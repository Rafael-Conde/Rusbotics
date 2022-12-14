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


use crate::{extract_robot_data_from_file, script};
use eframe::egui;
use egui_extras::image::RetainedImage;
use std::sync::{Arc, Mutex, MutexGuard};

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

#[derive(Default)]
struct MyApp
{
    comma_separated_data: String,
    picked_path: Option<Mutex<String>>,
    calculation_thread_state: Arc<Mutex<ThreadState>>,
    image_texture: Arc<Mutex<Option<RetainedImage>>>,
    missing_image_warned: bool,
}

#[derive(Default)]
enum ThreadState
{
    #[default]
    DidntRun,
    Running,
    Finished,
}

fn perform_calculations(path: String)
{
    let joints = extract_robot_data_from_file(path).unwrap() // turn this unwrap
    														 // into a pop-up warning
                                                   .to_dh_table()
                                                   .get_joints();
    script::get_matrix_image(joints); // later check is it's better to handle the error
    								  // here or in the caller, which I beliebe depends on
    								  // the success of this funciont
    								  //
    								  // this might not be an issue later since it's 
    								  // scheduled to change this and return the bytes of the image
    								  // directly to the retained image
}


fn button_generate_dh_matrix(picked_path: &MutexGuard<String>, calculation_thread_state: Arc<Mutex<ThreadState>>, image_texture: Arc<Mutex<Option<RetainedImage>>>)
{
    let temp = (*picked_path).clone();
    std::thread::spawn(move || {
        perform_calculations(temp);
        let path =  std::path::Path::new("test-page-0.png");
        if path.exists()
        {
			#[allow(clippy::option_if_let_else)]
            if let Ok(image_file_bytes) = std::fs::read(path)
            {
                if let Ok(mut image_texture) = image_texture.lock()
                {
                    *image_texture = Some(RetainedImage::from_image_bytes("equacao", &image_file_bytes).unwrap()); // turn this unwrap
                    																							   // into a pop-up warning
                }
            }
            else
            {
                println!("error while reading the image from the file");
            }
            if let Ok(mut state) = calculation_thread_state.lock()
            {
                *state = ThreadState::Finished;
            };
        }
    });
    // *current_thread_state = ThreadState::Running;
}



impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open file???").clicked()
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
                                    // teh deref is necessary
                                    // otherwise the Arc would've been copied
                                    // let temp = (*picked_path).clone();
                                    let calculation_thread_state =
                                        Arc::clone(&self.calculation_thread_state);
                                    let image_texture = Arc::clone(&self.image_texture);
                                    button_generate_dh_matrix(&picked_path, calculation_thread_state, image_texture);
                                //     std::thread::spawn(move || {
                                //         perform_calculations(temp);
                                //         let path =  std::path::Path::new("test-page-0.png");
                                //         if path.exists()
                                //         {
                                //             if let Ok(image_file_bytes) = std::fs::read(path)
                                //             {
                                //                 if let Ok(mut image_texture) = image_texture.lock()
                                //                 {
                                //                     *image_texture = Some(RetainedImage::from_image_bytes("equacao", &image_file_bytes).unwrap());
                                //                 }
                                //             }
                                //             else
                                //             {
                                //                 println!("error while reading the image from the file");
                                //             }
                                //             if let Ok(mut state) = calculation_thread_state.lock()
                                //             {
                                //                 *state = ThreadState::Finished;
                                //             };
                                //         }
                                //     });
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
                            // ui.add_enabled(false, egui::Button::new("Running calculations..."));
                            // ui.add_enabled(false, egui::ProgressBar::new(0.99f32).desired_width(230f32).text("Running Calculations...").animate(true));
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
                                    // teh deref is necessary
                                    // otherwise the Arc would've been copied
                                    // let temp = (*picked_path).clone();
                                    // let calculation_thread_state =
                                    //     Arc::clone(&self.calculation_thread_state);
                                    // let image_texture = Arc::clone(&self.image_texture);
                                    // std::thread::spawn(move || {
                                    //     perform_calculations(temp);
                                    //     let path =  std::path::Path::new("test-page-0.png");
                                    //     if path.exists()
                                    //     {
                                    //         if let Ok(image_file_bytes) = std::fs::read(path)
                                    //         {
                                    //             if let Ok(mut image_texture) = image_texture.lock()
                                    //             {
                                    //                 *image_texture = Some(RetainedImage::from_image_bytes("equacao", &image_file_bytes).unwrap());
                                    //             }
                                    //         }
                                    //         else
                                    //         {
                                    //             println!("error while reading the image from the file");
                                    //         }
                                    //         if let Ok(mut state) = calculation_thread_state.lock()
                                    //         {
                                    //             *state = ThreadState::Finished;
                                    //         };
                                    //     }
                                    // });
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
                                        image.show(ui);
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
