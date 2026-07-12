// Card grid panel: 4x13 grid + 2 Jokers on the right.
// Filtered cards are re-packed from top-left — no gaps.
// Grid fills available height, mouse-wheel to scroll.

use egui::{Align2, Color32, Painter, Pos2, ProgressBar, Rect, RichText, Ui, Vec2};
use crate::deck::{Card, Deck, Rank, Suit, make_reference_cards, TOTAL_CARDS};
use crate::effects::{draw_glow_border, ParticleSystem};
use crate::lang::T;

const CARD_SIZE: f32 = 72.0;
const CARD_GAP: f32 = 8.0;
const JOKER_W: f32 = 110.0;
const JOKER_H: f32 = 72.0;
const COLS: usize = 4;

const BG_UNCOLLECTED: Color32 = Color32::from_rgb(240, 237, 228);
const BORDER_UNCOLLECTED: Color32 = Color32::from_rgb(180, 175, 165);
const TEXT_UNCOLLECTED: Color32 = Color32::from_rgb(140, 135, 125);
const BG_COLLECTED: Color32 = Color32::from_rgb(255, 245, 210);
const TEXT_COLLECTED: Color32 = Color32::from_rgb(50, 45, 35);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    All, Collected, Uncollected, Suit(Suit),
}

impl FilterMode {
    fn label(&self, t: &T) -> String {
        match self {
            FilterMode::All => t.all().into(),
            FilterMode::Collected => t.collected().into(),
            FilterMode::Uncollected => t.uncollected().into(),
            FilterMode::Suit(Suit::Spades) => "♠".into(),
            FilterMode::Suit(Suit::Hearts) => "♥".into(),
            FilterMode::Suit(Suit::Diamonds) => "♦".into(),
            FilterMode::Suit(Suit::Clubs) => "♣".into(),
            _ => unreachable!(),
        }
    }
}

fn parse_rank(s: &str) -> Option<Rank> {
    match s.trim().to_uppercase().as_str() {
        "A"|"1"=>Some(Rank::Ace),"2"=>Some(Rank::Two),"3"=>Some(Rank::Three),"4"=>Some(Rank::Four),
        "5"=>Some(Rank::Five),"6"=>Some(Rank::Six),"7"=>Some(Rank::Seven),"8"=>Some(Rank::Eight),
        "9"=>Some(Rank::Nine),"10"=>Some(Rank::Ten),"J"=>Some(Rank::Jack),"Q"=>Some(Rank::Queen),
        "K"=>Some(Rank::King),_=>None,
    }
}

fn matches_filter(collected: bool, suit: Suit, rank: Rank, f: FilterMode, rf: &str) -> bool {
    let ok = match f {
        FilterMode::All => true, FilterMode::Collected => collected,
        FilterMode::Uncollected => !collected, FilterMode::Suit(s) => suit == s,
    };
    ok && (rf.trim().is_empty() || parse_rank(rf).is_none_or(|t| rank == t))
}

fn visible_indices(deck: &Deck, refs: &[Card; TOTAL_CARDS], f: FilterMode, rf: &str, pinned: &[usize]) -> Vec<usize> {
    let mut v: Vec<usize> = (0..52)
        .filter(|&i| matches_filter(deck.cards[i].collected, refs[i].suit, refs[i].rank, f, rf)
                  || pinned.contains(&i))
        .collect();
    if f != FilterMode::All || !rf.trim().is_empty() {
        v.sort_by_key(|&i| {
            let c = refs[i];
            (rank_ord(c.rank), suit_ord(c.suit))
        });
    }
    v
}

fn rank_ord(r: Rank) -> u8 {
    match r { Rank::Ace=>0,Rank::Two=>1,Rank::Three=>2,Rank::Four=>3,Rank::Five=>4,
        Rank::Six=>5,Rank::Seven=>6,Rank::Eight=>7,Rank::Nine=>8,Rank::Ten=>9,
        Rank::Jack=>10,Rank::Queen=>11,Rank::King=>12,_=>13 }
}
fn suit_ord(s: Suit) -> u8 { match s { Suit::Spades=>0,Suit::Hearts=>1,Suit::Diamonds=>2,Suit::Clubs=>3,_=>4 } }
fn packed_pos(idx: usize) -> (usize, usize) { (idx % COLS, idx / COLS) }
fn original_pos(idx: usize) -> (usize, usize) { (idx / 13, idx % 13) }

pub fn show_card_grid(
    ui: &mut Ui, t: &T,
    deck: &mut Deck, particle_system: &mut ParticleSystem,
    filter: &mut FilterMode, rank_filter: &mut String, scroll: &mut f32,
    complete_deck: &mut Option<String>, pinned: &mut Vec<usize>,
) -> bool {
    let mut go_back = false;
    let reference = make_reference_cards();

    // Header
    ui.horizontal(|ui| {
        if ui.button(t.back()).clicked() { go_back = true; }
        ui.heading(RichText::new(&deck.name).size(22.0).color(Color32::GOLD));
    });
    let c = deck.collected_count();
    let pct = c as f32 / TOTAL_CARDS as f32;
    ui.horizontal(|ui| {
        ui.label(format!("{}: {}/{} ({:.0}%)", t.progress(), c, TOTAL_CARDS, pct * 100.0));
        ui.add(ProgressBar::new(pct).desired_width(200.0).animate(true));
    });
    ui.horizontal(|ui| {
        if ui.button(t.select_all()).clicked() { deck.collect_all(); }
        if ui.button(t.deselect_all()).clicked() { deck.uncollect_all(); }
    });

    // Filter
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(t.filter());
        for &f in &[
            FilterMode::All, FilterMode::Collected, FilterMode::Uncollected,
            FilterMode::Suit(Suit::Spades), FilterMode::Suit(Suit::Hearts),
            FilterMode::Suit(Suit::Clubs), FilterMode::Suit(Suit::Diamonds),
        ] {
            let sel = *filter == f;
            let b = if sel {
                egui::Button::new(RichText::new(f.label(t)).size(14.0).color(Color32::WHITE))
                    .fill(Color32::from_rgb(70, 70, 70))
            } else { egui::Button::new(RichText::new(f.label(t)).size(14.0)) };
            if ui.add_sized([50.0, 24.0], b).clicked() { *filter = f; pinned.clear(); }
        }
        ui.separator();
        ui.label(t.rank());
        ui.add_sized([40.0, 20.0], egui::TextEdit::singleline(rank_filter).hint_text("A-K"));
    });
    let visible = visible_indices(deck, &reference, *filter, rank_filter, pinned);
    if *filter != FilterMode::All || !rank_filter.trim().is_empty() {
        ui.label(RichText::new(t.showing(visible.len())).size(12.0).color(Color32::GRAY));
    }
    ui.separator();

    // ── Body ──
    let use_orig = *filter == FilterMode::All && rank_filter.trim().is_empty();
    let grid_w = COLS as f32 * (CARD_SIZE + CARD_GAP) + CARD_GAP;
    let joker_w = JOKER_W + CARD_GAP * 2.0;
    let body_h = ui.available_height().max(200.0);
    let content_h = (if use_orig { 13.0 } else { ((visible.len() + COLS - 1) / COLS).max(1) as f32 })
        * (CARD_SIZE + CARD_GAP) + CARD_GAP;

    let (body, _) = ui.allocate_exact_size(
        Vec2::new(grid_w + 8.0 + joker_w, body_h),
        egui::Sense::hover(),
    );
    let grid_rect = Rect::from_min_size(body.min, Vec2::new(grid_w, body_h));
    let joker_rect = Rect::from_min_size(Pos2::new(body.min.x + grid_w + 8.0, body.min.y), Vec2::new(joker_w, body_h));

    let view_h = body_h - CARD_GAP * 2.0;
    let max_s = (content_h - view_h).max(0.0);
    let ctx = ui.ctx();
    if ui.rect_contains_pointer(grid_rect) {
        *scroll -= ctx.input(|i| i.raw_scroll_delta.y) * 2.0;
    }
    *scroll = scroll.clamp(0.0, max_s);

    let was_all = deck.is_all_collected();

    let grid_pad = grid_rect.shrink(CARD_GAP);
    let grid_painter = ui.painter().with_clip_rect(grid_rect);
    let shifted = grid_pad.translate(Vec2::new(0.0, -*scroll));
    paint_grid(&grid_painter, deck, &reference, shifted, &visible, use_orig);
    handle_clicks(ui, deck, shifted, &visible, use_orig, pinned);

    // Jokers — label painted directly, no ui.put to avoid borrow conflict
    let jp = ui.painter();
    let jx = joker_rect.min.x + CARD_GAP;
    let jy0 = joker_rect.min.y + 2.0;
    jp.text(
        Pos2::new(jx + JOKER_W / 2.0, jy0),
        Align2::CENTER_TOP,
        t.jokers(),
        egui::FontId::proportional(14.0),
        Color32::from_gray(180),
    );
    for (ji, &idx) in [52usize, 53].iter().enumerate() {
        let collected = deck.cards[idx].collected;
        match *filter {
            FilterMode::Collected if !collected => continue,
            FilterMode::Uncollected if collected => continue,
            _ => {}
        }
        let y = jy0 + 20.0 + ji as f32 * (JOKER_H + CARD_GAP);
        let cr = Rect::from_min_size(Pos2::new(jx, y), Vec2::new(JOKER_W, JOKER_H));
        paint_card(&jp, cr, &reference[idx], collected);
        if ctx.input(|i| i.pointer.primary_clicked()) {
            if let Some(m) = ctx.input(|i| i.pointer.interact_pos()) {
                if cr.contains(m) {
                    deck.toggle_card(idx);
                    if !pinned.contains(&idx) { pinned.push(idx); }
                }
            }
        }
    }

    if !was_all && deck.is_all_collected() {
        *complete_deck = Some(deck.name.clone());
        particle_system.celebrate(Vec2::new(grid_rect.center().x, grid_rect.center().y));
    }

    for p in &particle_system.particles {
        let a = (p.lifetime / p.max_lifetime).min(1.0);
        ui.painter().circle_filled(
            Pos2::new(p.pos.x, p.pos.y), p.radius * a,
            Color32::from_rgba_premultiplied(p.color.r(), p.color.g(), p.color.b(), (a * 255.0) as u8),
        );
    }

    go_back
}

fn paint_grid(
    painter: &Painter, deck: &Deck, reference: &[Card; TOTAL_CARDS],
    rect: Rect, visible: &[usize], orig: bool,
) {
    let step = CARD_SIZE + CARD_GAP;
    for (vi, &idx) in visible.iter().enumerate() {
        let (col, row) = if orig { original_pos(idx) } else { packed_pos(vi) };
        let x = rect.min.x + col as f32 * step;
        let y = rect.min.y + row as f32 * step;
        paint_card(painter, Rect::from_min_size(Pos2::new(x, y), Vec2::new(CARD_SIZE, CARD_SIZE)),
            &reference[idx], deck.cards[idx].collected);
    }
}

fn handle_clicks(
    ui: &Ui, deck: &mut Deck, rect: Rect,
    visible: &[usize], orig: bool, pinned: &mut Vec<usize>,
) {
    let ctx = ui.ctx();
    if !ctx.input(|i| i.pointer.primary_clicked()) { return; }
    let Some(mouse) = ctx.input(|i| i.pointer.interact_pos()) else { return };
    let step = CARD_SIZE + CARD_GAP;
    for (vi, &idx) in visible.iter().enumerate() {
        let (col, row) = if orig { original_pos(idx) } else { packed_pos(vi) };
        let x = rect.min.x + col as f32 * step;
        let y = rect.min.y + row as f32 * step;
        if Rect::from_min_size(Pos2::new(x, y), Vec2::new(CARD_SIZE, CARD_SIZE)).contains(mouse) {
            deck.toggle_card(idx);
            if !pinned.contains(&idx) { pinned.push(idx); }
            break;
        }
    }
}

fn paint_card(painter: &Painter, rect: Rect, card: &Card, collected: bool) {
    if collected {
        painter.rect_filled(rect, 4.0, BG_COLLECTED);
        draw_glow_border(painter, rect, Color32::GOLD, 2.0, 1.0);
    } else {
        painter.rect_filled(rect, 4.0, BG_UNCOLLECTED);
        painter.rect_stroke(rect, 4.0, (1.0, BORDER_UNCOLLECTED), egui::StrokeKind::Middle);
    }

    let is_red = matches!(card.suit, Suit::Hearts | Suit::Diamonds);
    let tc = if is_red {
        if collected { Color32::from_rgb(180, 40, 40) } else { Color32::from_rgb(180, 140, 140) }
    } else {
        if collected { TEXT_COLLECTED } else { TEXT_UNCOLLECTED }
    };

    let short = match card.rank {
        Rank::RedJoker => "★ Red".into(), Rank::BlackJoker => "☆ Black".into(), _ => card.display(),
    };
    let fs = if rect.width() > 90.0 { 22.0 } else { 18.0 };
    painter.text(rect.center(), Align2::CENTER_CENTER, short, egui::FontId::proportional(fs), tc);
}
