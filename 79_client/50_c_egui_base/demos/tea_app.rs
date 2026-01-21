use super::*;
use eframe::CreationContext;

pub trait Tea_App_T {
    type Msg;

    fn update_state(&mut self, msg: Self::Msg);

    fn run_one_frame(
        &mut self,
        ctx: &egui::Context,
        tx: &mut Push_Take<Self::Msg>,

        frame: &mut eframe::Frame,
    );
}

/*
struct Tea_App<T: Tea_App_T> {
    model: T,
}

impl<T: Tea_App_T> eframe::App for Tea_App<T> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut msgs = Push_Take::new();
        self.model.run_one_frame(ctx, &mut msgs, _frame);
        for msg in msgs.take() {
            self.model.update_state(msg);
        }
    }
} */

pub fn run<
    T: eframe::App + 'static,
    F: FnOnce(&CreationContext) -> T + 'static,
>(
    f: F,
) {
    #[cfg(target_arch = "wasm32")]
    {
        use eframe::wasm_bindgen::JsCast as _;
        let web_options = eframe::WebOptions::default();
        wasm_bindgen_futures::spawn_local(async move {
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

            let canvas = document
                .get_element_by_id("main_canvas")
                .expect("Failed to find main_canvas")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("main_canvas was not a HtmlCanvasElement");

            let start_result = eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(move |cc| Ok(Box::new(f(cc)))),
                )
                .await;
        });
    }
}
