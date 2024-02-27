use eframe::App;

use crate::app::ChessGame;
use crate::common::game_modes::GameType;
pub fn local(chess_game: &mut ChessGame, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("local_options").show(ctx, |ui| {
        ui.heading("Local Game Options");
        ui.label("Time Control");
        ui.horizontal(|ui| {
            ui.radio_value(&mut chess_game.game_type, Some(GameType::Blitz), "Blitz");
            ui.radio_value(&mut chess_game.game_type, Some(GameType::Rapid), "Rapid");
            ui.radio_value(
                &mut chess_game.game_type,
                Some(GameType::Classical),
                "Classical",
            );
            ui.radio_value(&mut chess_game.game_type, Some(GameType::Bullet), "Bullet");
        });
        ui.spacing();
        ui.label("use fen string");
        ui.text_edit_multiline(&mut chess_game.local_fen_string);
        ui.spacing();
        if ui
            .button("Start Game")
            .on_hover_text("Start a local game")
            .clicked()
        {
            chess_game.start_local_game();
            if !chess_game.local_fen_string.is_empty() {
                chess_game.client.update_fen(&chess_game.local_fen_string);
            }
        };
    });
}
