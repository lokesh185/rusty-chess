use crate::common::game_modes::{GameMode, GameType};
use crate::common::logic::ChessPosition;
use crate::images::PieceImages;
use crate::{client::GameClient, game_components, login_components};
use egui::{Color32, Pos2};
use egui_extras::install_image_loaders;

const DARK: Color32 = Color32::from_rgb(177, 110, 65);
const LIGHT: Color32 = Color32::from_rgb(255, 213, 153);

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

pub struct ChessGame {
    page: Page,
    pub game_type: Option<GameType>,
    pub local_fen_string: String,
    pub online_fen_string: String,

    pub client: GameClient,
    #[serde(skip)]
    pub images: PieceImages<'static>,

    pub color_black: Color32,
    pub color_white: Color32,

    pub mouse_pos: Option<Pos2>,
    pub pos_held: Option<ChessPosition>,
    pub tile_width: f32,
}
impl PartialEq for ChessGame {
    fn eq(&self, other: &Self) -> bool {
        self.game_type == other.game_type
            && self.local_fen_string == other.local_fen_string
            && self.online_fen_string == other.online_fen_string
    }
}
impl ChessGame {
    pub fn start_local_game(&mut self) {
        if let Some(game_type) = self.game_type {
            self.client = GameClient::new_local(game_type);
            self.page = Page::InGame(LoginInfo {
                game_id: "local".to_string(),
                user_id: "local".to_string(),
            });
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
enum Page {
    Login,
    InGame(LoginInfo),
}
#[derive(serde::Deserialize, serde::Serialize)]
struct LoginInfo {
    game_id: String,
    user_id: String,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self {
            mouse_pos: None,
            page: Page::Login,
            game_type: None,
            local_fen_string: String::default(),
            online_fen_string: String::default(),
            client: GameClient::default(),
            images: PieceImages::default(),
            color_black: DARK,
            color_white: LIGHT,
            pos_held: None,
            tile_width: 100.0,
        }
    }
}

impl ChessGame {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        install_image_loaders(&cc.egui_ctx);
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        log::info!("the app is starting for the first time");
        Default::default()
    }
}

impl eframe::App for ChessGame {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self
            .client
            .game_mode
            .is_some_and(|game_mode| game_mode == GameMode::Local)
        {
            self.client
                .clock
                .update_time(self.client.game_state.active_color);
        }

        egui::TopBottomPanel::top("top_menu_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
                egui::reset_button(ui, self);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("game");
            match &mut self.page {
                Page::Login => login_components::local(self, ctx, _frame),
                Page::InGame(_) => game_components::full(self, ctx, _frame),
            }
        });
    }
}
