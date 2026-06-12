use eframe::egui;

fn bold_text(text: &str) -> egui::RichText {
    egui::RichText::new(text).strong()
}

struct PlayerRanks {
    player_name: String,
    ranked_1s: String,
    ranked_2s: String,
    ranked_3s: String,
}

pub struct RankDisplayApp {
    players: Option<Vec<PlayerRanks>>,
}

impl Default for RankDisplayApp {
    fn default() -> Self {
        RankDisplayApp { players: None }
    }
}

impl eframe::App for RankDisplayApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("player ranks");
            ui.add_space(8.0);
            if let Some(players) = &self.players {
                egui::Grid::new("player list")
                    .num_columns(4)
                    .spacing([12.0, 12.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(bold_text("Name"));
                        ui.label(bold_text("1s"));
                        ui.label(bold_text("2s"));
                        ui.label(bold_text("3s"));
                        ui.end_row();

                        for player in players {
                            ui.label(&player.player_name);
                            ui.label(&player.ranked_1s);
                            ui.label(&player.ranked_2s);
                            ui.label(&player.ranked_3s);
                            ui.end_row();
                        }
                    });
            } else {
                ui.label("no players");
                if ui.button("click to make players exist").clicked() {
                    self.players = Some(vec![
                        PlayerRanks {
                            player_name: String::from("zen"),
                            ranked_1s: String::from("ssl"),
                            ranked_2s: String::from("ssl"),
                            ranked_3s: String::from("gc1 div 4"),
                        },
                        PlayerRanks {
                            player_name: String::from("Rw9"),
                            ranked_1s: String::from("ssl"),
                            ranked_2s: String::from("ssl"),
                            ranked_3s: String::from("gc2 div 1"),
                        },
                    ]);
                }
            }
        });
    }
}
