use crate::app::ChessGame;
use crate::common::game_modes::GameType;
pub fn local(chess_game: &mut ChessGame, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // ui.horizontal(|ui| {
    //     ui.spacing_mut().item_spacing.x = 0.0;
    //     ui.label("Powered by ");
    //     ui.hyperlink_to("egui", "https://github.com/emilk/egui");
    //     ui.label(" and ");
    //     ui.hyperlink_to(
    //         "eframe",
    //         "https://github.com/emilk/egui/tree/master/crates/eframe",
    //     );
    //     ui.label(".");
    // });
    // left side panel with different times
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
            self.game
        };
    });
}
