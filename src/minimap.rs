// Minimapa en la esquina superior derecha: paredes, jugador,
// cartas restantes y portal.

use crate::level::Level;
use crate::player::Player;
use crate::sprites::{Kind, Sprite};
use macroquad::prelude::*;

pub fn draw_minimap(level: &Level, player: &Player, sprites: &[Sprite], portal_active: bool) {
    let cell = 7.0;
    let w = level.w as f32 * cell;
    let h = level.h as f32 * cell;
    let ox = screen_width() - w - 14.0;
    let oy = 14.0;

    draw_rectangle(ox - 4.0, oy - 4.0, w + 8.0, h + 8.0, Color::new(0.02, 0.02, 0.05, 0.72));
    draw_rectangle_lines(ox - 4.0, oy - 4.0, w + 8.0, h + 8.0, 2.0, Color::new(0.9, 0.8, 0.5, 0.9));

    let wall_colors = [
        Color::new(0.85, 0.82, 0.75, 0.95),
        Color::new(0.75, 0.45, 0.30, 0.95),
        Color::new(0.85, 0.75, 0.35, 0.95),
        Color::new(0.60, 0.45, 0.30, 0.95),
        Color::new(0.45, 0.42, 0.55, 0.95),
    ];
    for y in 0..level.h {
        for x in 0..level.w {
            let c = level.cells[y * level.w + x];
            if c > 0 {
                draw_rectangle(
                    ox + x as f32 * cell,
                    oy + y as f32 * cell,
                    cell,
                    cell,
                    wall_colors[(c as usize - 1).min(4)],
                );
            }
        }
    }

    for s in sprites.iter().filter(|s| s.alive) {
        let (color, r) = match s.kind {
            Kind::Card => (Color::new(1.0, 0.85, 0.2, 1.0), 2.6),
            Kind::Spirit => (Color::new(0.6, 0.8, 1.0, 0.8), 2.0),
            Kind::Portal => {
                if portal_active {
                    (Color::new(0.7, 0.3, 1.0, 1.0), 3.0)
                } else {
                    (Color::new(0.4, 0.25, 0.5, 0.8), 3.0)
                }
            }
        };
        draw_circle(ox + s.x * cell, oy + s.y * cell, r, color);
    }

    // jugador: punto + dirección de vista
    let px = ox + player.x * cell;
    let py = oy + player.y * cell;
    let (dx, dy) = player.dir_vec();
    draw_line(px, py, px + dx * 10.0, py + dy * 10.0, 2.0, Color::new(0.3, 1.0, 0.4, 0.9));
    draw_circle(px, py, 3.2, Color::new(0.3, 1.0, 0.4, 1.0));
}
