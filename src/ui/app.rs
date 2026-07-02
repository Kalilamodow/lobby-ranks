use super::{hotkey, matches::Matches};
use eframe::egui;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

fn bold_text(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).strong()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Panel {
    Matches,
}

const ALL_PANELS: [Panel; 1] = [Panel::Matches];

pub struct RlBuddyApp {
    error_receiver: mpsc::Receiver<String>,
    current_error: Option<String>,
    prev_hide_pos: Option<egui::Pos2>,
    overlay_rx: mpsc::Receiver<bool>,

    open_panels: HashSet<Panel>,
    matches: Matches,
}

impl RlBuddyApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let ctx = cc.egui_ctx.clone();
        let (errors_tx, errors_rx) = mpsc::channel();
        let (overlay_tx, overlay_rx) = mpsc::channel();

        let app = RlBuddyApp {
            error_receiver: errors_rx,
            current_error: None,
            overlay_rx,
            prev_hide_pos: None,

            open_panels: HashSet::from([Panel::Matches]),
            matches: Matches::new(&ctx, overlay_tx.clone(), errors_tx),
        };

        let overlay_tx_for_hotkey = overlay_tx.clone();
        let ctx_for_hotkey = ctx.clone();
        thread::spawn(move || {
            hotkey::listen_for_hotkey(overlay_tx_for_hotkey, ctx_for_hotkey);
        });

        app
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.prev_hide_pos = ctx.input(|i| {
            i.viewport()
                .outer_rect
                .map(|outer_rect| egui::pos2(outer_rect.left(), outer_rect.top()))
        });

        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(8.0, 8.0)));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::AlwaysOnTop,
        ));
    }

    fn hide(&self, ctx: &egui::Context) {
        if let Some(move_to) = self.prev_hide_pos {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(move_to));
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::AlwaysOnBottom,
        ));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::Normal,
        ));
    }

    fn panel_add_button(&mut self, ui: &mut egui::Ui, text: &str, panel: Panel) {
        if ui
            .add_enabled(!self.open_panels.contains(&panel), egui::Button::new(text))
            .clicked()
        {
            self.open_panels.insert(panel);
        }
    }

    fn panel_remove_button(&mut self, ui: &mut egui::Ui, text: &str, panel: &Panel) {
        if ui.button(text).clicked() {
            self.open_panels.remove(panel);
        }
    }
}

impl eframe::App for RlBuddyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if let Ok(new_error) = self.error_receiver.try_recv() {
            self.current_error = Some(new_error);
        }

        egui::Panel::bottom("bottom_buttons").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                for panel in ALL_PANELS {
                    match panel {
                        Panel::Matches => {
                            self.panel_add_button(ui, "Matches", panel);
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(err) = &self.current_error {
                ui.label(bold_text("Fatal error"));
                ui.label(err);
                if ui.button("Exit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            } else {
                ui.vertical_centered_justified(|ui| {
                    let mut is_first = true;

                    for panel in ALL_PANELS {
                        if self.open_panels.contains(&panel) {
                            if !is_first {
                                ui.separator();
                            }
                            is_first = false;

                            match panel {
                                Panel::Matches => {
                                    self.panel_remove_button(ui, "Matches", &panel);
                                    ui.add(&self.matches);
                                }
                            }
                        }
                    }
                });
            }
        });
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(should_overlay) = self.overlay_rx.try_recv() {
            if should_overlay {
                self.show(ctx);
            } else {
                self.hide(ctx);
            }
        }

        self.matches.logic(ctx);
    }
}
