use iyes_loopless::prelude::{ConditionSet, NextState};

use crate::game::GameResult;
use crate::*;

pub struct GameEndedPlugin;

impl Plugin for GameEndedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::GameEnded)
                .with_system(draw_game_over_screen)
                .into(),
        );
    }
}

fn draw_game_over_screen(
    mut commands: Commands,
    // mut audio: EventWriter<SoundEvent>,
    mut egui_context: ResMut<EguiContext>,
    windows: ResMut<Windows>,
    result: Res<State<GameResult>>,
) {
    let win_fill = egui_context.ctx_mut().style().visuals.window_fill();
    let text_col = egui_context.ctx_mut().style().visuals.text_color();
    let window = windows.get_primary().unwrap();
    let win_ht = window.height();
    let win_wi = window.width();
    let height = 220.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(egui_context.ctx_mut(), |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                // window border
                rect.shrink(2.0),
                5.0,
                win_fill,
                Stroke::new(1.0, text_col),
            );
            painter.text(
                // title text
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                match *result.current() {
                    GameResult::Won => "The Ogre Necromancer is dead! You win!",
                    GameResult::Lost => "You lost! Keep Sir Hoardalot alive!",
                },
                FontId::proportional(46.0),
                text_col,
            );
            painter.line_segment(
                // divider
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_col),
            );

            let quit_btn = ui.put(
                egui::Rect::from_center_size(egui::pos2(win_wi / 2., win_ht / 2. + 132.), egui::vec2(280., 66.)),
                egui::Button::new("Back to menu"),
            );
            if quit_btn.clicked() {
                commands.insert_resource(NextState(AppState::MainMenu));
            }
        });
}
