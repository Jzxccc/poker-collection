#![windows_subsystem = "windows"]

// Poker Collection Tracker
// A Windows desktop app for tracking playing card collections built with egui.

mod app;
mod deck;
mod effects;
mod storage;
mod ui;

use app::App;

fn icon_data() -> egui::IconData {
    // Generate a simple 32x32 poker card icon
    let size = 32;
    let mut rgba = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        for x in 0..size {
            let cx = x as f32 - 16.0;
            let cy = y as f32 - 16.0;
            let dist = ((cx * cx + cy * cy).sqrt() / 16.0).clamp(0.0, 1.0);

            // Rounded rect card shape
            let in_rect = cx.abs() < 14.0 && cy.abs() < 14.5;
            let in_round = in_rect && (cx.abs() < 10.0 || cy.abs() < 10.0 || dist < 0.85);

            if in_round && cx.abs() < 14.0 && cy.abs() < 14.0 {
                // Card background: dark green
                rgba.extend_from_slice(&[34, 94, 44, 255]);
                // Simple spade/heart pattern in center
                let h = cy / 14.0;
                let w = cx / 14.0;
                // Draw a simple spade ▲ shape in white
                if h < -0.2 && h > -0.95 && w.abs() < (-h - 0.2) * 0.8 {
                    rgba.extend_from_slice(&[255, 255, 255, 255]);
                } else {
                    rgba.extend_from_slice(&[34, 94, 44, 255]);
                }
            } else {
                // Transparent
                rgba.extend_from_slice(&[0, 0, 0, 0]);
            }
        }
    }
    egui::IconData { rgba, width: size as u32, height: size as u32 }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Poker Collection")
            .with_icon(icon_data()),
        ..Default::default()
    };
    eframe::run_native(
        "Poker Collection",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
