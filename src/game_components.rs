use crate::app::ChessGame;
use crate::common::logic::{ChessMove, ChessPosition};
use egui::Vec2;
use egui::{Pos2, Rect};

pub fn full(chess_game: &mut ChessGame, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    chess_game.mouse_pos = ctx.input(|i| i.pointer.latest_pos());
    egui::SidePanel::left("previous_moves_and_time").show(ctx, |ui| {
        ui.heading("Time ");
        egui::Grid::new("time_grid_clock").show(ui, |ui| {
            ui.label("White");
            ui.label("Black");
            ui.end_row();
            ui.label(chess_game.client.clock.white_time.to_string());
            ui.label(chess_game.client.clock.black_time.to_string());
            ui.end_row();
        });
        ui.heading("Previous Moves");
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("prev_move_grid").show(ui, |ui| {
                for (i, chess_move) in chess_game.client.game_state.prev_moves.iter().enumerate() {
                    ui.label(i.to_string());
                    ui.label(chess_move.to_string());
                    ui.end_row();
                }
            });
        });
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        chess_board(chess_game, ui, ctx);
    });
}

fn chess_board(chess_game: &mut ChessGame, ui: &mut egui::Ui, ctx: &egui::Context) {
    let end_pos = ui.clip_rect().right_bottom();
    let size = ui.available_size();
    let start_pos = Pos2::new(end_pos.x - size.x, 24.);
    let rct = Rect::from_two_pos(start_pos, end_pos);
    let tile_side = rct.width().min(rct.height()) / 8.;
    chess_game.tile_width = tile_side;
    let mut cur_chess_pos = match chess_game.mouse_pos {
        Some(cur_mouse_pos) => {
            let x = cur_mouse_pos - start_pos;
            if x.x.is_sign_positive() && x.y.is_sign_positive() {
                ChessPosition::new((x.x / (tile_side)) as i32, (x.y / (tile_side)) as i32)
            } else {
                None
            }
        }
        None => None,
    };
    cur_chess_pos = if let Some(ccp) = cur_chess_pos {
        Some(ccp.adjust_for_current_player(chess_game.client.game_state.active_color))
    } else {
        None
    };
    if ctx.input(|i| i.pointer.primary_pressed()) {
        if chess_game.pos_held.is_none() {
            if cur_chess_pos.is_some_and(|chess_pos| {
                chess_game
                    .client
                    .game_state
                    .board
                    .get_piece_at_pos(&chess_pos)
                    .is_some_and(|piece| {
                        piece.player_kind == chess_game.client.game_state.active_color
                    })
            }) {
                chess_game.pos_held = cur_chess_pos;
            }
        }
    }
    let move_result = if ctx.input(|i| i.pointer.primary_released()) {
        if let (Some(from_pos), Some(to_pos)) = (chess_game.pos_held.clone(), cur_chess_pos) {
            chess_game.pos_held = None;

            chess_game
                .client
                .game_state
                .make_move(&ChessMove::new(from_pos, to_pos))
        } else {
            None
        }
    } else {
        None
    };

    paint_board(chess_game, ui, start_pos, tile_side);
}
fn paint_board(chess_game: &mut ChessGame, ui: &mut egui::Ui, start: Pos2, tile_side: f32) {
    // check if piece in hand
    let painter = ui.painter();
    let mut red_squares: Vec<ChessPosition> = vec![];
    let mut o_piece = None;
    let mut o_piece_pos = None;
    if let Some(mut piece_pos) = chess_game.pos_held {
        piece_pos = piece_pos;
        if let Some(piece) = chess_game
            .client
            .game_state
            .board
            .get_piece_at_pos(&piece_pos)
        {
            red_squares.append(&mut chess_game.client.possible_moves(&piece_pos));
            o_piece = Some(piece);
            o_piece_pos = Some(piece_pos);
        }
    }
    // draw the board and pieces

    for pos in ChessPosition::iter() {
        let color = if (pos.file + pos.rank) % 2 == 0 {
            chess_game.color_white
        } else {
            chess_game.color_black
        };
        let rect = egui::Rect::from_min_max(
            egui::Pos2::new(pos.file as f32 * tile_side, pos.rank as f32 * tile_side),
            egui::Pos2::new(
                (pos.file + 1) as f32 * tile_side,
                (pos.rank + 1) as f32 * tile_side,
            ),
        )
        .translate(Vec2::new(start.x, start.y));
        if red_squares
            .contains(&pos.adjust_for_current_player(chess_game.client.game_state.active_color))
        {
            painter.rect_filled(rect, 0., egui::Color32::RED);
        } else {
            painter.rect_filled(rect, 0., color);
        }
        if let Some(piece) = chess_game.client.game_state.board.get_piece_at_pos(
            &pos.adjust_for_current_player(chess_game.client.game_state.active_color),
        ) {
            if !o_piece_pos.is_some_and(|pos2| {
                pos == pos2.adjust_for_current_player(chess_game.client.game_state.active_color)
            }) {
                let image = chess_game.images.get(&piece);

                image.paint_at(ui, rect);
            }
        }
    }
    if let Some(piece) = o_piece {
        let image = chess_game.images.get(&piece);
        image.paint_at(
            ui,
            Rect::from_center_size(
                chess_game.mouse_pos.unwrap_or(Pos2::new(0., 0.)),
                Vec2::new(chess_game.tile_width * 1.2, chess_game.tile_width * 1.2),
            ),
        );
    }
}
