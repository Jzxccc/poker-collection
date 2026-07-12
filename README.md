# Poker Collection

A desktop app for tracking playing card collections, built with Rust + egui.

Track multiple decks of 54 playing cards (52 standard + 2 Jokers). Click cards to mark them as collected, with visual effects and progress tracking.

## Features

- **Multi-Deck Support** — create and manage multiple decks independently
- **Click to Collect** — tap any card to toggle collected/uncollected status
- **Visual Effects** — collected cards glow with gold borders; full-collection celebration particles
- **Smart Filters** — filter by collection status, suit, or rank (A-K)
- **Pinned Toggle** — toggled cards stay visible when filtering, no surprise disappear
- **Progress Tracking** — real-time progress bar and percentage per deck
- **Select All / Deselect All** — one-click to mark entire deck
- **Scrollable Grid** — mouse wheel to scroll through all cards
- **Chinese / English** — one-click language toggle
- **Auto-Save** — data persists to `%LOCALAPPDATA%\poker-collection\poker_collection.json`
- **Custom Icon** — place `icon.png` or `icon.jpg` in project root for custom exe icon

## Screenshots

```
┌──────────────────────────────────────────┐
│  Poker Collection / 扑克牌收集    [中/EN] │
├──────────────────────────────────────────┤
│  Your Decks / 你的牌盒         [+ New]    │
│  ┌──────────────────────────────────────┐│
│  │ Deck 1         20/54 (37%)  [Delete] ││
│  │ Deck 2         54/54 (100%) [Delete] ││
│  └──────────────────────────────────────┘│
└──────────────────────────────────────────┘

┌────────────────────────────────────┬─────┐
│ Back                              │Jokers│
│ Progress: 20/54 (37%) ████░░░░    │     │
│ [Select All] [Deselect All]       │ ★ Red│
│ Filter: [All][Collected][UnCol] ♠♥♣♦│ ☆ Blk│
│ Rank: [A-K]                       │     │
│                                   │     │
│ ♠A ♥A ♣A ♦A                      │     │
│ ♠2 ♥2 ♣2 ♦2                      │     │
│ ♠3 ♥3 ♣3 ♦3  (scroll with wheel) │     │
│  ...                              │     │
└────────────────────────────────────┴─────┘
```

## Build

Requires [Rust](https://rustup.rs/) toolchain.

```bash
cargo build --release
```

Output: `target/release/poker-collection.exe`

### Custom Icon

Place a `icon.png` or `icon.jpg` in the project root before building. The build script auto-converts it to ICO format.

## Tech Stack

- [egui](https://github.com/emilk/egui) — immediate mode GUI
- [serde](https://serde.rs/) — JSON serialization
- [image](https://crates.io/crates/image) — icon format conversion

## License

MIT
