// Application state and main egui panel layout.
// Coordinates navigation between deck list view and card grid detail view.

use eframe::egui;
use egui::CentralPanel;
use crate::deck::Deck;
use crate::effects::ParticleSystem;
use crate::lang::{Lang, T};
use crate::storage;
use crate::ui::{deck_list, card_grid};
use crate::ui::card_grid::FilterMode;

/// Which panel is currently shown.
enum Panel {
    DeckList,
    CardGrid { deck_index: usize },
}

pub struct App {
    decks: Vec<Deck>,
    next_id: u64,
    panel: Panel,
    particle_system: ParticleSystem,
    lang: Lang,
    // Deck list state
    show_create_dialog: bool,
    new_deck_name: String,
    // Card grid state
    card_filter: FilterMode,
    rank_filter: String,
    grid_scroll: f32,
    complete_deck: Option<String>,
    pinned: Vec<usize>,
    // Auto-save tracking
    needs_save: bool,
}

impl App {
    pub fn new() -> Self {
        let decks = storage::load_decks();
        let next_id = decks.iter().map(|d| d.id).max().map(|m| m + 1).unwrap_or(1);
        Self {
            decks,
            next_id,
            panel: Panel::DeckList,
            particle_system: ParticleSystem::new(),
            lang: Lang::Zh,
            show_create_dialog: false,
            new_deck_name: String::new(),
            card_filter: FilterMode::All,
            rank_filter: String::new(),
            grid_scroll: 0.0,
            complete_deck: None,
            pinned: Vec::new(),
            needs_save: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.particle_system.update(dt);
        if self.particle_system.active {
            ctx.request_repaint();
        }

        let t = T::new(self.lang);

        CentralPanel::default().show(ctx, |ui| {
            match self.panel {
                Panel::DeckList => {
                    let prev_len = self.decks.len();
                    if let Some(deck_index) = deck_list::show_deck_list(
                        ui, &t,
                        &mut self.decks,
                        &mut self.next_id,
                        &mut self.show_create_dialog,
                        &mut self.new_deck_name,
                        &mut self.lang,
                    ) {
                        self.card_filter = FilterMode::All;
                        self.rank_filter = String::new();
                        self.grid_scroll = 0.0;
                        self.pinned.clear();
                        self.panel = Panel::CardGrid { deck_index };
                    }
                    if prev_len != self.decks.len() {
                        self.needs_save = true;
                    }
                }
                Panel::CardGrid { deck_index } => {
                    let prev_count = self.decks[deck_index].collected_count();
                    let go_back = card_grid::show_card_grid(
                        ui, &t,
                        &mut self.decks[deck_index],
                        &mut self.particle_system,
                        &mut self.card_filter,
                        &mut self.rank_filter,
                        &mut self.grid_scroll,
                        &mut self.complete_deck,
                        &mut self.pinned,
                    );
                    if prev_count != self.decks[deck_index].collected_count() {
                        self.needs_save = true;
                    }
                    if go_back {
                        self.panel = Panel::DeckList;
                    }
                }
            }
        });

        // Completion popup
        if let Some(ref name) = self.complete_deck.clone() {
            egui::Window::new(t.collection_complete())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(egui::RichText::new(t.congratulations()).size(24.0));
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(t.fully_collected(name)).size(16.0));
                        ui.add_space(12.0);
                        if ui.button(t.ok()).clicked() {
                            self.complete_deck = None;
                        }
                    });
                });
        }

        if self.needs_save {
            storage::save_decks(&self.decks);
            self.needs_save = false;
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        storage::save_decks(&self.decks);
    }
}
