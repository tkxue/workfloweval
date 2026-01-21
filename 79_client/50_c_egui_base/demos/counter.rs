use super::*;

pub struct Counter {
    cnt: i32,
    name: String,
    age: i32,
}

pub enum Counter_Msg {
    Inc,
    Dec,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            cnt: 0,
            name: "".to_string(),
            age: 0,
        }
    }
}

impl Tea_App_T for Counter {
    type Msg = Counter_Msg;

    fn update_state(&mut self, msg: Self::Msg) {
        match msg {
            Counter_Msg::Inc => self.cnt += 1,
            Counter_Msg::Dec => self.cnt -= 1,
        }
    }

    fn run_one_frame(
        &mut self,
        ctx: &Context,
        tx: &mut Push_Take<Self::Msg>,

        frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Increment Age").clicked() {
                self.age += 1;
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
