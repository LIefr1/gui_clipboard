use eframe::egui;


pub struct Counter {
    pub (crate) counter: i32
} 


impl eframe::App for Counter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello World!");
        if ui.button("Increment").clicked() {
            self.counter += 1;
        }
        ui.label(format!("Counter: {}", self.counter));
    
        });
    }
}
