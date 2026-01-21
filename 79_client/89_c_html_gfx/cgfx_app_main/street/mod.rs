use super::*;

use c_app_ssheet::shared::sparse::ui::SparseTableState;
use c_app_ssheet::shared::spreadsheet::ui::SpreadsheetState;
use egui::{Ui, ViewportBuilder};
use std::default::Default;

mod kml;
mod places;
mod plugins;
mod tiles;
mod windows;

use egui::{
    Button, CentralPanel, Context, DragPanButtons, Frame, OpenUrl, Rect, Vec2,
};
use tiles::{TilesKind, providers};
use walkers::{Map, MapMemory};

use tiles::Providers;

pub struct Cgfx_Street {
    is_open: bool,
    providers: Providers,
    map_memory: MapMemory,
    click_watcher: plugins::ClickWatcher,
    zoom_with_ctrl: bool,
}

impl Cgfx_Street {
    pub fn new(egui_ctx: Context) -> Self {
        c_egui_base::egui_extras::install_image_loaders(&egui_ctx);
        Self {
            is_open: true,
            providers: providers(egui_ctx.to_owned()),
            map_memory: MapMemory::default(),
            click_watcher: Default::default(),
            zoom_with_ctrl: false,
        }
    }
}

impl Cgfx_App_T for Cgfx_Street {
    fn update(&mut self, ui: &egui::Ui) {
        let max_rect = ui.max_rect();
        let ctx = ui.ctx();
        egui::Window::new(self.name())
            .open(&mut self.is_open)
            .default_width(1000.0)
            .default_height(800.0)
            .constrain_to(max_rect)
            .show(ui.ctx(), |ui| {


            let my_position = walkers::lon_lat(-73.9822, 40.7685);

            let tiles = self
                .providers
                .available
                .get_mut(&self.providers.selected)
                .unwrap();
            let attributions: Vec<_> = tiles
                .iter()
                .map(|tile| tile.as_ref().attribution())
                .collect();

            // In egui, widgets are constructed and consumed in each frame.
            let mut map = Map::new(None, &mut self.map_memory, my_position);

            // Various aspects of the map can be configured.
            map = map
                .zoom_with_ctrl(self.zoom_with_ctrl)
                .drag_pan_buttons(DragPanButtons::PRIMARY | DragPanButtons::SECONDARY);

            // Optionally, plugins can be attached.
            map = map
                .with_plugin(plugins::places())
                .with_plugin(plugins::CustomShapes {})
                .with_plugin(&mut self.click_watcher)
                // .with_plugin(kml::poland_borders())
                // .with_plugin(kml::outgym_umea_layer())
                ;

            // Multiple layers can be added.
            for (n, tiles) in tiles.iter_mut().enumerate() {
                // With a different transparency.
                let transparency = if n == 0 { 1.0 } else { 0.25 };
                map = map.with_layer(tiles.as_mut(), transparency);
            }

            // Draw the map widget.
            let response = map.show(ui, |ui, _, projector, _| {
                // You can add any additional contents to the map's UI here.
                let bastion = projector.project(places::bastion_sakwowy()).to_pos2();
                ui.put(
                    Rect::from_center_size(bastion, Vec2::new(140., 20.)),
                    Button::new("Bastion Sakwowy"),
                )
                .on_hover_text("Click to see some information about this place.")
                .clicked()
                .then_some("https://www.wroclaw.pl/dla-mieszkanca/bastion-sakwowy-wroclaw-atrakcje")
            });

            // Could have done it in the closure, but this way you can see how to pass values outside.
            if let Some(url) = response.inner {
                ctx.open_url(OpenUrl::new_tab(url));
            }

            // Draw utility windows.
            /*
            {
                use windows::*;

                zoom(ui, &mut self.map_memory);
                go_to_my_position(ui, &mut self.map_memory);
                self.click_watcher.show_position(ui);

                let http_stats = tiles
                    .iter()
                    .filter_map(|tiles| {
                        if let TilesKind::Http(tiles) = tiles {
                            Some(tiles.stats())
                        } else {
                            None
                        }
                    })
                    .collect();

                // controls(self, ui, http_stats );
                acknowledge(ui, attributions);
            }
            */
    });
    }

    fn name(&self) -> &'static str {
        "Street"
    }
}

/*
pub struct Cgfx_Street {
    is_open: bool,
    state: SpreadsheetState,
}

impl Cgfx_Street {
    pub fn new() -> Self {
        Self {
            is_open: true,
            state: SpreadsheetState::default(),
        }
    }
}

impl Cgfx_App_T for Cgfx_Street {
    fn name(&self) -> &'static str {
        "Street"
    }

    fn update(&mut self, ui: &egui::Ui) {
        let max_rect = ui.max_rect();
        egui::Window::new(self.name())
            .open(&mut self.is_open)
            .default_width(400.0)
            .constrain_to(max_rect)
            .show(ui.ctx(), |ui| {
                let state = &mut self.state;
                egui::Resize::default()
                    .min_size((100.0, 100.0))
                    .default_size((1200.0, 1000.0))
                    .max_size((1024.0, 768.0))
                    .show(ui, |ui| {
                        let (_response, actions) =
                            c_app_ssheet::shared::spreadsheet::ui::show_table(
                                ui, state,
                            );
                        c_app_ssheet::shared::spreadsheet::ui::handle_actions(
                            actions, state,
                        );
                    });
                ui.separator();
                ui.label("content below");
            });
    }
}
 */
