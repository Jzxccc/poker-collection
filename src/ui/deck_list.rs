// Deck list panel: shows all decks with progress, add/rename/delete controls.

use egui::{Align2, Color32, ProgressBar, RichText, Ui, Window};

use crate::deck::Deck;
use crate::lang::{Lang, T};

pub fn show_deck_list(
    ui: &mut Ui,
    t: &T,
    decks: &mut Vec<Deck>,
    next_id: &mut u64,
    show_create_dialog: &mut bool,
    new_deck_name: &mut String,
    lang: &mut Lang,
) -> Option<usize> {
    let mut open_deck: Option<usize> = None;

    // Title row with language toggle
    ui.horizontal(|ui| {
        ui.heading(RichText::new(t.app_title()).size(24.0).color(Color32::GOLD));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(RichText::new(lang.label()).size(12.0)).clicked() {
                *lang = lang.toggle();
            }
        });
    });

    ui.separator();

    ui.horizontal(|ui| {
        ui.heading(t.your_decks());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(t.new_deck()).clicked() {
                *show_create_dialog = true;
                *new_deck_name = String::new();
            }
        });
    });

    ui.separator();

    if decks.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new(t.no_decks()).size(16.0));
        });
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut to_delete: Option<usize> = None;

        for (idx, deck) in decks.iter().enumerate() {
            let collected = deck.collected_count();
            let total = deck.total_cards();
            let progress = collected as f32 / total as f32;

            ui.horizontal(|ui| {
                // Deck name button
                if ui
                    .add_sized(
                        [180.0, 30.0],
                        egui::Button::new(RichText::new(&deck.name).size(16.0)),
                    )
                    .clicked()
                {
                    open_deck = Some(idx);
                }

                // Progress
                ui.label(format!("{}/{} ({:.0}%)", collected, total, progress * 100.0));
                ui.add(ProgressBar::new(progress).desired_width(100.0).show_percentage());

                // Push delete button to right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(t.delete()).clicked() {
                        to_delete = Some(idx);
                    }
                });
            });

            ui.add_space(4.0);
        }

        if let Some(idx) = to_delete {
            decks.remove(idx);
        }
    });

    // Create deck dialog
    if *show_create_dialog {
        let mut should_close = false;
        Window::new(t.new_deck_title())
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.label(t.deck_name());
                ui.text_edit_singleline(new_deck_name);
                ui.horizontal(|ui| {
                    if ui.button(t.create()).clicked() && !new_deck_name.trim().is_empty() {
                        let id = *next_id;
                        *next_id += 1;
                        decks.push(Deck::new(id, new_deck_name.trim()));
                        should_close = true;
                    }
                    if ui.button(t.cancel()).clicked() {
                        should_close = true;
                    }
                });
            });
        if should_close {
            *show_create_dialog = false;
        }
    }

    open_deck
}
