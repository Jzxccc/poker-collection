// Visual effects: particles for celebration, glow for collected cards.

use egui::{Color32, Rect, Vec2};
use rand::Rng;

/// A single particle in the celebration effect.
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub color: Color32,
    pub radius: f32,
}

/// Particle system for celebration explosions.
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub active: bool,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            active: false,
        }
    }

    /// Spawn celebration particles centered at `center` in screen space.
    pub fn celebrate(&mut self, center: Vec2) {
        let mut rng = rand::thread_rng();
        for _ in 0..60 {
            let angle = rng.r#gen::<f32>() * std::f32::consts::TAU;
            let speed = rng.gen_range(50.0..300.0);
            let lifetime = rng.gen_range(1.0..2.5);
            let colors = [
                Color32::GOLD,
                Color32::from_rgb(255, 200, 0),
                Color32::from_rgb(255, 100, 50),
                Color32::from_rgb(255, 50, 100),
                Color32::from_rgb(100, 200, 255),
                Color32::from_rgb(50, 255, 100),
            ];
            let color = colors[rng.gen_range(0..colors.len())];
            self.particles.push(Particle {
                pos: center,
                vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                lifetime,
                max_lifetime: lifetime,
                color,
                radius: rng.gen_range(2.0..6.0),
            });
        }
        self.active = true;
    }

    /// Update particle lifetimes. Called each frame with delta time in seconds.
    pub fn update(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            p.lifetime -= dt;
            p.pos += p.vel * dt;
            p.vel *= 0.96; // friction
        }
        self.particles.retain(|p| p.lifetime > 0.0);
        if self.particles.is_empty() {
            self.active = false;
        }
    }
}

/// Draw glow border around a rect.
pub fn draw_glow_border(painter: &egui::Painter, rect: Rect, color: Color32, thickness: f32, glow_strength: f32) {
    let glow_color = Color32::from_rgba_premultiplied(
        color.r(),
        color.g(),
        color.b(),
        (glow_strength * 0.4 * 255.0) as u8,
    );
    for i in 0..4 {
        let expand = thickness * (i as f32 + 1.0) * 0.5;
        let alpha = 0.15 * glow_strength * (1.0 - i as f32 * 0.25);
        let c = Color32::from_rgba_premultiplied(
            glow_color.r(),
            glow_color.g(),
            glow_color.b(),
            (alpha * 255.0) as u8,
        );
        painter.rect_stroke(rect.expand(expand), 1.0, (thickness * 0.5, c), egui::StrokeKind::Middle);
    }
    painter.rect_stroke(rect, 0.0, (thickness, color), egui::StrokeKind::Middle);
}
