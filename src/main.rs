#![windows_subsystem = "windows"]

// Poker Collection Tracker
// A Windows desktop app for tracking playing card collections built with egui.

mod app;
mod deck;
mod effects;
mod lang;
mod storage;
mod ui;

use app::App;

fn setup_fonts(ctx: &egui::Context) {
    // egui's ttf-parser supports TTC directly — pass raw data
    let paths = [
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "C:\\Windows\\Fonts\\Deng.ttf",
    ];
    for p in paths {
        if let Ok(data) = std::fs::read(p) {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert("cjk".into(), std::sync::Arc::new(egui::FontData::from_owned(data)));
            fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "cjk".into());
            fonts.families.entry(egui::FontFamily::Monospace).or_default().insert(0, "cjk".into());
            ctx.set_fonts(fonts);
            return;
        }
    }
}

fn icon_data() -> egui::IconData {
    let size = 32;
    let mut rgba = Vec::with_capacity(size * size * 4);
    for y in 0..size {
        for x in 0..size {
            let cx = x as f32 - 16.0;
            let cy = y as f32 - 16.0;
            let in_rect = cx.abs() < 14.0 && cy.abs() < 14.5;
            if in_rect {
                let h = cy / 14.0;
                let w = cx / 14.0;
                if h < -0.2 && h > -0.95 && w.abs() < (-h - 0.2) * 0.8 {
                    rgba.extend_from_slice(&[255, 255, 255, 255]);
                } else {
                    rgba.extend_from_slice(&[34, 94, 44, 255]);
                }
            } else {
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
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}
