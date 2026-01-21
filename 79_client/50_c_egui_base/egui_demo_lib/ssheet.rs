use crate::egui_extras::{Column, TableBuilder};
use egui::color_picker::Alpha;
use egui::mutex::{Mutex, RwLock};
use egui::scroll_area::ScrollBarVisibility;
use egui::special_emojis::GITHUB;
use egui::widgets::TextEdit;
use egui::{
    CollapsingHeader, Color32, Context, Key, Label, OpenUrl, Pos2, Rect, Response, RichText,
    ScrollArea, Sense, StrokeKind, Ui, Vec2, Window,
};
use ehttp::{Request, streaming};
use ewebsock::WsSender;
use ewebsock::{WsEvent, WsMessage, WsReceiver};
use gloo_timers::callback::Timeout;
use log::error;
use log::{debug, trace, warn};
use lru::LruCache;
use serde_json::Deserializer;
use serde_json::json;
use std::cell::RefCell;
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::ops::ControlFlow;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;
// ==========================================================================

/// The cell as it comes from the backend.
#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
pub(crate) struct Cell {
    pub(crate) id: u64,
    pub(crate) raw_value: String,
    pub(crate) computed_value: String,
    pub(crate) background: i32,
}

/// A request to update a cell.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
pub(crate) struct UpdateCellRequest {
    pub(crate) id: u64,
    pub(crate) raw_value: String,
    pub(crate) background: i32,
}

impl From<&CellContent> for UpdateCellRequest {
    fn from(cell: &CellContent) -> Self {
        Self {
            id: cell.id,
            raw_value: cell.write_buffer.read().clone(),
            background: cell.background.load(Ordering::Relaxed),
        }
    }
}

/// A Cell that we currently track as part of the spreadsheet.
pub(crate) struct CellContent {
    pub(crate) id: u64,
    pub(crate) content: RwLock<String>,
    pub(crate) write_buffer: RwLock<String>,
    pub(crate) old_write_buffer: Mutex<String>,
    pub(crate) background: AtomicI32,
    pub(crate) is_editing: AtomicBool,
    debounce_bg_change: Rc<Mutex<Debouncer>>,
}

/// We convert Cells from the backend into CellContent that we can edit.
impl From<Cell> for CellContent {
    fn from(cell: Cell) -> Self {
        Self {
            id: cell.id,
            content: RwLock::new(cell.computed_value),
            write_buffer: RwLock::new(cell.raw_value.clone()),
            old_write_buffer: Mutex::new(cell.raw_value),
            is_editing: AtomicBool::new(false),
            background: AtomicI32::new(cell.background),
            debounce_bg_change: Rc::new(Mutex::new(Debouncer::new())),
        }
    }
}

impl CellContent {
    /// A new empty cell.
    pub(crate) fn empty(id: u64) -> Self {
        Self {
            id,
            write_buffer: RwLock::new(String::new()),
            old_write_buffer: Mutex::new(String::new()),
            content: RwLock::new(String::new()),
            is_editing: AtomicBool::new(false),
            background: AtomicI32::new(i32::from_le_bytes(Color32::TRANSPARENT.to_array())),
            debounce_bg_change: Rc::new(Mutex::new(Debouncer::new())),
        }
    }

    pub(crate) fn background_color(&self) -> Color32 {
        let rgba_premultiplied = i32::to_le_bytes(self.background.load(Ordering::Relaxed));
        Color32::from_rgba_premultiplied(
            rgba_premultiplied[0],
            rgba_premultiplied[1],
            rgba_premultiplied[2],
            rgba_premultiplied[3],
        )
    }

    pub(crate) fn is_editing(&self) -> bool {
        self.is_editing.load(Ordering::SeqCst)
    }

    /// We set the cell into edit mode -- if the user clicks it.
    pub(crate) fn edit(&self) {
        let mut old_value = self.old_write_buffer.lock();
        old_value.clear();
        old_value.push_str(&self.write_buffer.read());
        self.is_editing.store(true, Ordering::SeqCst);
    }

    /// We disable editing mode -- if the user clicks elsewhere.
    pub(crate) fn disable_edit(&self, revert: bool) {
        if revert {
            let old_value = self.old_write_buffer.lock();
            let mut write_buffer = self.write_buffer.write();
            write_buffer.clear();
            write_buffer.push_str(&old_value);
        }
        self.is_editing.store(false, Ordering::SeqCst);
    }

    pub(crate) fn set_background(&self, color: Color32) {
        self.background
            .store(i32::from_le_bytes(color.to_array()), Ordering::Relaxed);
        let mut debouncer = self.debounce_bg_change.lock();
        let cell_update = self.into();
        debouncer.debounce(Duration::from_millis(350), move || {
            update_cell(
                format!(
                    "{}/api/spreadsheet",
                    CellCache::API_HOST.unwrap_or("http://localhost:3000")
                ),
                cell_update,
            );
        });
    }

    pub(crate) fn save(&self) {
        use e_api::*;
        let mut old_value = self.old_write_buffer.lock();
        let new_value = self.write_buffer.read();
        if *old_value != *new_value {
            /*
            update_cell(
                format!(
                    "{}/api/spreadsheet",
                    CellCache::API_HOST.unwrap_or("http://localhost:3000")
                ),
                self.into(),
            );

             */
            old_value.clear();
            old_value.push_str(&new_value);

            {
                let mut content_value = self.content.write();
                content_value.clear();
                content_value.push_str(&new_value);
            }
        }
        wlog!("saving data");
    }

    /// We render the cell in the UI/Table.
    pub fn ui(&self, ui: &mut Ui) -> Response {
        if self.is_editing() {
            let mut content = self.write_buffer.write();
            ui.add(TextEdit::singleline(&mut *content))
        } else {
            let content = self.content.read().to_string();
            ui.add(Label::new(&content).sense(Sense::click()))
        }
    }
}

/// Sends a PATCH request to the server to update a cell.
fn update_cell(url: String, data: UpdateCellRequest) {
    /*
    let request = Request::json(url, &data).unwrap();
    ehttp::fetch(request, move |response| {
        if let Ok(response) = response {
            if !response.ok {
                warn!("POST request failed: {:?}", response.text());
            }
        } else {
            debug!("No response received");
        }
    });
    */
}

/// Helper to display CellContent.
impl Display for CellContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content.read())
    }
}

pub(crate) struct Loader {
    pub(crate) is_open: AtomicBool,
    ws_sender: Mutex<WsSender>,
}

impl Loader {
    pub(crate) fn new(ws_sender: WsSender) -> Self {
        Self {
            ws_sender: Mutex::new(ws_sender),
            is_open: AtomicBool::new(false),
        }
    }

    pub(crate) fn fetch(&self, range: Range<u64>) -> bool {
        if !self.is_open.load(Ordering::Relaxed) {
            return false;
        }

        let mut sender = self.ws_sender.lock();
        sender.send(WsMessage::Text(
            json!({"from": range.start, "to": range.end}).to_string(),
        ));
        true
    }
}

/// The CellCache stores a fixed number of cells in memory.
///
/// - It fetches cells from the backend as needed.
/// - It always contains the cells that the user is currently looking at (and some more
///   since it also prefetches cells around the current view to make scrolling smooth).
/// - It debounces fetching of new rows to avoid fetching too many cells at once.
pub(crate) struct CellCache {
    cells: Rc<Mutex<LruCache<u64, Rc<CellContent>>>>,
    fetcher: Arc<Loader>,
    debouncer: Rc<RefCell<Debouncer>>,
    current_range: Option<Range<u64>>,
    prefetch_before_after_id: u64,
    max_cells: usize,
}

impl CellCache {
    pub(crate) const API_HOST: Option<&'static str> = option_env!("API_HOST");

    pub fn new(fetcher: Arc<Loader>, width: usize, height: usize) -> Self {
        let prefetch_before_after_id = 100 * width as u64;
        let lru_cache_size = NonZeroUsize::new(200 * width).unwrap();

        Self {
            fetcher,
            cells: Rc::new(Mutex::new(LruCache::new(lru_cache_size))),
            debouncer: Rc::new(RefCell::new(Debouncer::new())),
            current_range: None,
            prefetch_before_after_id,
            max_cells: width * height,
        }
    }

    pub fn set(&mut self, id: u64, c: CellContent) {
        let mut cells = self.cells.lock();
        cells.push(id, Rc::new(c));
    }

    pub fn get(&mut self, id: u64) -> Rc<CellContent> {
        let mut cells = self.cells.lock();

        if let Some(c) = cells.get(&id) {
            c.clone()
        } else {
            let c = Rc::new(CellContent::empty(id));
            cells.push(id, c.clone());

            if let Some(current_range) = &self.current_range {
                if current_range.contains(&id) {
                    // Already fetching this range...
                    return c;
                }
            }

            let start = id.saturating_sub(self.prefetch_before_after_id);
            let end = std::cmp::min(
                id.saturating_add(self.prefetch_before_after_id),
                self.max_cells as u64,
            );
            let current_range = start..end;
            self.current_range = Some(current_range.clone());
            trace!("fetching range: {:?}", current_range);
            let fetcher = self.fetcher.clone();

            let debouncer_clone = self.debouncer.clone();
            debouncer_clone
                .borrow_mut()
                .debounce(Duration::from_millis(100), move || {
                    let mut max_retry = 10;
                    while !fetcher.fetch(current_range.clone()) && max_retry > 0 {
                        max_retry -= 1;
                    }
                });

            c
        }
    }
}

// ==========================================================================

use crate::egui_demo_lib::Demo;

#[derive(serde::Deserialize, Default, Debug, Clone)]
pub struct Stats {
    pub filled_total: u64,
    pub filled_this_hour: u64,
    pub filled_today: u64,
    pub filled_this_week: u64,
    pub currently_active_users: u64,
}

pub struct SpreadsheetApp {
    focused_row: usize,
    focused_col: usize,
    bg_color_picked: Color32,
    last_key_time: f64,
    num_cols: usize,
    num_rows: usize,
    loader: Arc<Loader>,
    ws_receiver: WsReceiver,
    stats: Arc<RwLock<Stats>>,
    cell_cache: CellCache,
    editing_cell: Option<u64>,
    reference_open: bool,
}

impl Demo for SpreadsheetApp {
    fn name(&self) -> &'static str {
        "DataCalc"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_width(400.0)
            .show(ctx, |ui| {
                self.ui(ui); // ui.ctx());
                // self.ui(ui);
            });
    }
}

impl SpreadsheetApp {
    const DEFAULT_COLS: usize = 52;
    const DEFAULT_ROWS: usize = 1_000_000; // 26*40_000_000 = 1_040_000_000 cells
    const DEFAULT_ROW_HEIGHT: f32 = 18.0;

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        crate::egui_extras::install_image_loaders(&cc.egui_ctx);
        let server = CellCache::API_HOST.unwrap_or("http://localhost:3000");

        // Refresh stats
        let stats = Arc::new(RwLock::new(Stats::default()));

        let (ws_sender, ws_receiver) = {
            let egui_ctx = cc.egui_ctx.clone();
            let wakeup = move || egui_ctx.request_repaint();
            let url = format!("{}/api/spreadsheet", server);
            ewebsock::connect_with_wakeup(&url, Default::default(), wakeup).unwrap()
        };
        let loader = Arc::new(Loader::new(ws_sender));

        SpreadsheetApp {
            focused_row: 0,
            focused_col: 0,
            bg_color_picked: Color32::TRANSPARENT,
            last_key_time: 0.0,
            num_cols: Self::DEFAULT_COLS,
            num_rows: Self::DEFAULT_ROWS,
            stats,
            loader: loader.clone(),
            ws_receiver,
            cell_cache: CellCache::new(loader, Self::DEFAULT_COLS, Self::DEFAULT_ROWS),
            editing_cell: None,
            reference_open: false,
        }
    }
}

impl SpreadsheetApp {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let original_spacing = {
                    let style = ui.style_mut();
                    let original_spacing = style.spacing.item_spacing;
                    style.spacing.item_spacing.x = 2.0;
                    original_spacing
                };

                ui.label("Set Background Color");
                ui.colored_label(Color32::LIGHT_BLUE, RichText::new("[?]")).on_hover_text(
                    "By default colors are at 0 alpha (fully transparent).\nMove the bottom slider in the widget to decrease the transparency if yo want\nto set a color on a new transparent cell.",
                );
                let style = ui.style_mut();
                style.spacing.item_spacing = original_spacing;
            });

            let id = self.focused_row as u64 * self.num_cols as u64 + self.focused_col as u64;
            let cell = self.cell_cache.get(id);
            let color_response = egui::widgets::color_picker::color_edit_button_srgba(
                ui,
                &mut self.bg_color_picked,
                Alpha::BlendOrAdditive,
            );
            if color_response.changed() {
                cell.set_background(self.bg_color_picked);
            }
        });

        /*
        ScrollArea::both()
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                */
        ScrollArea::horizontal().show(ui, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .resizable(false)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                // .column(Column::remainder())
                .columns(
                    Column::initial(100.0)
                        .at_least(50.0)
                        .resizable(true)
                        .clip(true),
                    self.num_cols,
                )
                .header(Self::DEFAULT_ROW_HEIGHT + 3.0, |mut header| {
                    let col_idx_to_label = |idx: usize| {
                        if idx < 26 {
                            format!("{}", (b'A' + idx as u8) as char)
                        } else {
                            format!(
                                "{}{}",
                                (b'A' + (idx / 26 - 1) as u8) as char,
                                (b'A' + (idx % 26) as u8) as char
                            )
                        }
                    };

                    header.col(|ui| {
                        ui.strong("");
                    });

                    for col_index in 0..self.num_cols {
                        header.col(|ui| {
                            ui.strong(col_idx_to_label(col_index));
                        });
                    }
                })
                .body(|body| {
                    body.rows(Self::DEFAULT_ROW_HEIGHT, self.num_rows, |mut row| {
                        let row_index = row.index();
                        row.col(|ui| {
                            ui.strong(row_index.to_string());
                        });

                        for col_index in 0..self.num_cols {
                            let id = row_index as u64 * self.num_cols as u64 + col_index as u64;
                            let cell = self.cell_cache.get(id);
                            row.col(|ui| {
                                let has_focus =
                                    row_index == self.focused_row && col_index == self.focused_col;
                                let rect = ui.available_rect_before_wrap();
                                let resp = ui.interact(
                                    ui.available_rect_before_wrap(),
                                    ui.make_persistent_id(id),
                                    Sense::click(),
                                );
                                ui.painter().rect_filled(rect, 0.0, cell.background_color());
                                let cell_response = cell.ui(ui);

                                // Adjust cell focus based on the new coordinates
                                if has_focus {
                                    ui.painter().rect_stroke(
                                        rect,
                                        0.0,
                                        egui::Stroke::new(1.0, Color32::LIGHT_BLUE),
                                        StrokeKind::Outside,
                                    );
                                }

                                ui.input(|i| {
                                    const KEY_DELAY: f64 = 0.01;
                                    let now = i.time;
                                    i.events.iter().for_each(|i| {
                                        if let egui::Event::Key { key, pressed, .. } = i {
                                            if now - self.last_key_time > KEY_DELAY && *pressed {
                                                match key {
                                                    Key::Escape => {
                                                        if self.editing_cell.is_some() {
                                                            cell.disable_edit(true);
                                                        }
                                                    }
                                                    Key::Enter => {
                                                        self.focused_row = (self.focused_row + 1)
                                                            .min(self.num_rows - 1);
                                                        self.last_key_time = now;
                                                    }
                                                    Key::ArrowDown => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_row = (self.focused_row
                                                                + 1)
                                                            .min(self.num_rows - 1);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    Key::ArrowUp => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_row =
                                                                self.focused_row.saturating_sub(1);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    Key::ArrowRight => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_col = (self.focused_col
                                                                + 1)
                                                            .min(self.num_cols - 1);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    Key::ArrowLeft => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_col =
                                                                self.focused_col.saturating_sub(1);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    Key::PageDown => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_row = (self.focused_row
                                                                + 10)
                                                                .min(self.num_rows - 1);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    Key::PageUp => {
                                                        if self.editing_cell.is_none() {
                                                            self.focused_row =
                                                                self.focused_row.saturating_sub(10);
                                                            self.last_key_time = now;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    });
                                });

                                // Set focus on the cell
                                if resp.clicked()
                                    || (cell_response.clicked() && !cell_response.has_focus())
                                {
                                    self.focused_row = row_index;
                                    self.focused_col = col_index;
                                    self.bg_color_picked = cell.background_color();
                                }

                                // Done with editing
                                if self.editing_cell.is_some() && cell_response.lost_focus() {
                                    cell.disable_edit(false);
                                    cell.save();
                                    self.editing_cell = None;
                                }

                                // Edit the current cell
                                if self.editing_cell.is_none()
                                    && (resp.double_clicked()
                                        || cell_response.double_clicked()
                                        || (resp.has_focus()
                                            && ui.input(|i| i.key_pressed(Key::Enter))))
                                {
                                    cell_response.request_focus();
                                    cell.edit();
                                    self.editing_cell = Some(id);
                                }
                            });
                        }
                    });
                });
        });
    }
}

pub fn streaming_request(
    url: String,
    handle_data: Arc<dyn Fn(String) -> ControlFlow<()> + Send + Sync>,
) {
    let remainder = Arc::new(Mutex::new(String::new()));

    // Handle a chunk of data received from the server, this might not be a complete JSON object
    // so we need to store the remainder of the last chunk and append it to the next chunk
    let handle_chunk: Arc<dyn Fn(Vec<u8>) -> ControlFlow<()> + Send + Sync> =
        Arc::new(move |chunk: Vec<u8>| {
            if chunk.is_empty() {
                return ControlFlow::Break(());
            }
            let mut remainder = remainder.lock();

            let mut current_chunk = remainder.to_string();
            current_chunk.extend(String::from_utf8_lossy(chunk.as_slice()).chars());

            // For ndjson, needs to end with a newline, if not it's an incomplete chunk
            // store the last bit in the remainder
            if !current_chunk.ends_with('\n') {
                // split off the last chunk that doesn't end with a newline
                let (chunk_str, new_remainder) = match current_chunk.rfind('\n') {
                    Some(idx) => current_chunk.split_at(idx + 1),
                    None => {
                        *remainder = current_chunk;
                        return ControlFlow::Continue(());
                    }
                };
                *remainder = new_remainder.to_string();
                current_chunk = chunk_str.to_string();
            } else {
                *remainder = String::new();
            }

            handle_data(current_chunk)
        });

    let request = Request::get(url.clone());
    streaming::fetch(request, move |result: ehttp::Result<streaming::Part>| {
        let part = match result {
            Ok(part) => part,
            Err(err) => {
                error!("an error occurred while streaming `{url}`: {err}");
                return ControlFlow::Break(());
            }
        };

        match part {
            streaming::Part::Response(response) => {
                if response.ok {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(())
                }
            }
            streaming::Part::Chunk(chunk) => handle_chunk(chunk),
        }
    });
}

pub(crate) struct Debouncer {
    timeout: Option<Timeout>,
}

impl Debouncer {
    pub(crate) fn new() -> Self {
        Self { timeout: None }
    }

    pub(crate) fn debounce<F>(&mut self, delay: Duration, callback: F)
    where
        F: 'static + FnOnce(),
    {
        if let Some(timeout) = self.timeout.take() {
            timeout.cancel();
        }

        self.timeout = Some(Timeout::new(delay.as_millis() as u32, callback));
    }
}
