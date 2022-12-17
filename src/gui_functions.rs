use crate::{extract_robot_data_from_file, script};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub trait Gui
{
    fn run_gui(&self);
}

pub fn get_gui() -> Box<dyn Gui>
{
    Box::new(MyApp::default())
}

impl Gui for MyApp
{
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
    let joints = extract_robot_data_from_file(path).unwrap()
                                                   .to_dh_table()
                                                   .get_joints();
    script::get_matrix_image(joints);
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
                    match *current_thread_state
                    {
                        ThreadState::DidntRun =>
                        {
                            if ui.button("Generate equation").clicked()
                            {
                                if let Ok(mutex) = picked_path.lock()
                                {
                                    let temp = (*mutex).clone();
                                    let calculation_thread_state =
                                        Arc::clone(&self.calculation_thread_state);
                                    std::thread::spawn(move || {
                                        perform_calculations(temp);
                                        if let Ok(mut state) = calculation_thread_state.lock()
                                        {
                                            *state = ThreadState::Finished;
                                        };
                                    });
                                }
                                *current_thread_state = ThreadState::Running;
                            }
                        }
                        ThreadState::Running =>
                        {
                            ui.add_enabled(false, egui::Button::new("Running calculations..."));
                        }
                        ThreadState::Finished =>
                        {
                            if ui.button("Rerun calculations").clicked()
                            {
                                if let Ok(mut picked_path) = picked_path.lock()
                                {
                                    // teh deref is necessary
                                    // otherwise the Arc would've been copied
                                    let temp = (*picked_path).clone();
                                    let calculation_thread_state =
                                        Arc::clone(&self.calculation_thread_state);
                                    std::thread::spawn(move || {
                                        perform_calculations(temp);
                                        if let Ok(mut state) = calculation_thread_state.lock()
                                        {
                                            *state = ThreadState::Finished;
                                        };
                                    });
                                }
                                *current_thread_state = ThreadState::Running;
                            }
                        }
                    }
                }
            }
        });
    }
}
