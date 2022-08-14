use eframe::egui;
use egui::plot::{Line, Plot, Value, Values};

pub struct PlotWindow {
    name: String,
    age: u32,
    sin_incr: u32,
}

impl Default for PlotWindow {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            sin_incr: 0,
        }
    }
}

impl eframe::App for PlotWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // Plot window
            let sin = (0..1000).map(|i| {
                let x = f64::from(i) * 0.001 * f64::from(self.sin_incr);
                Value::new(x, x.sin())
            });

            let anti_sin = sin.clone().map(|val| Value::new(val.x, val.y * -1.0));

            let line = Line::new(Values::from_values_iter(sin));
            let anti_line = Line::new(Values::from_values_iter(anti_sin));
            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.line(anti_line);
            });

            self.sin_incr += 1;
        });

        ctx.request_repaint();
    }
}
