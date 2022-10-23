use eframe::egui;

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
    cs_data: String,
    picked_path: Option<String>,
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
                                                 self.picked_path =
                                                     Some(path.display().to_string());
                                             }
                                         }
                                         ui.horizontal(|ui| {
                                               ui.label("Insert data here: ");
                                               ui.text_edit_singleline(&mut self.cs_data);
                                           });
                                         if let Some(picked_path) = &self.picked_path
                                         {
                                             ui.horizontal(|ui| {
                                                   ui.label("Picked file:");
                                                   ui.monospace(picked_path);
                                               });
                                         }
                                     });
    }
}
