// Interfaz: menú de bienvenida con selección de arcano, HUD en juego,
// pantalla de éxito por nivel y pantalla de victoria final.

use macroquad::prelude::*;

pub const CARD_NAMES: [&str; 3] = ["THE FOOL", "THE HANGED MAN", "THE HERMIT"];
pub const CARD_NUMERALS: [&str; 3] = ["0", "XII", "IX"];
pub const CARD_COLORS: [Color; 3] = [
    Color::new(0.98, 0.83, 0.30, 1.0),
    Color::new(0.35, 0.75, 0.80, 1.0),
    Color::new(0.55, 0.55, 0.85, 1.0),
];

fn center_text(text: &str, y: f32, size: u16, color: Color) {
    let m = measure_text(text, None, size, 1.0);
    draw_text(text, (screen_width() - m.width) / 2.0, y, size as f32, color);
}

fn starfield(t: f32) {
    for i in 0..90u32 {
        let h = (i.wrapping_mul(2654435761)) as f32;
        let x = (h % 1000.0) / 1000.0 * screen_width();
        let y = ((h / 7.0) % 1000.0) / 1000.0 * screen_height();
        let tw = 0.4 + 0.6 * ((t * 1.3 + i as f32).sin() * 0.5 + 0.5);
        draw_circle(x, y, 1.4, Color::new(0.9, 0.9, 1.0, 0.5 * tw));
    }
}

/// dibuja una carta de tarot del menú; sel = resaltada
fn draw_tarot_card(x: f32, y: f32, w: f32, h: f32, idx: usize, sel: bool, t: f32) {
    let lift = if sel { (t * 3.0).sin() * 5.0 - 12.0 } else { 0.0 };
    let y = y + lift;
    let accent = CARD_COLORS[idx];
    if sel {
        let g = 0.5 + 0.5 * (t * 4.0).sin();
        draw_rectangle(x - 8.0, y - 8.0, w + 16.0, h + 16.0, Color::new(accent.r, accent.g, accent.b, 0.25 + 0.2 * g));
    }
    draw_rectangle(x, y, w, h, Color::new(0.10, 0.07, 0.16, 1.0));
    draw_rectangle_lines(x, y, w, h, 4.0, accent);
    draw_rectangle_lines(x + 8.0, y + 8.0, w - 16.0, h - 16.0, 2.0, Color::new(accent.r, accent.g, accent.b, 0.5));

    let cx = x + w / 2.0;
    let cy = y + h * 0.42;
    match idx {
        0 => {
            // El Loco: sol y camino al precipicio
            draw_circle(cx, cy - 20.0, 22.0, accent);
            for i in 0..8 {
                let a = i as f32 / 8.0 * std::f32::consts::TAU + t * 0.4;
                draw_line(cx + a.cos() * 28.0, cy - 20.0 + a.sin() * 28.0,
                          cx + a.cos() * 38.0, cy - 20.0 + a.sin() * 38.0, 3.0, accent);
            }
            draw_triangle(vec2(cx - 40.0, cy + 55.0), vec2(cx + 8.0, cy + 15.0), vec2(cx + 40.0, cy + 55.0),
                          Color::new(0.8, 0.7, 0.55, 1.0));
        }
        1 => {
            // El Colgado: figura invertida colgando
            draw_line(cx - 30.0, cy - 38.0, cx + 30.0, cy - 38.0, 5.0, Color::new(0.5, 0.35, 0.2, 1.0));
            let sway = (t * 1.5).sin() * 4.0;
            draw_line(cx, cy - 38.0, cx + sway, cy - 6.0, 3.0, accent);
            draw_circle(cx + sway, cy + 34.0, 12.0, accent); // cabeza abajo
            draw_line(cx + sway, cy - 6.0, cx + sway, cy + 22.0, 6.0, accent);
            draw_circle(cx + sway, cy + 34.0, 16.0, Color::new(accent.r, accent.g, accent.b, 0.3)); // halo
        }
        _ => {
            // El Ermitaño: figura con farol
            draw_triangle(vec2(cx - 22.0, cy + 50.0), vec2(cx, cy - 30.0), vec2(cx + 22.0, cy + 50.0),
                          Color::new(0.35, 0.35, 0.5, 1.0));
            draw_circle(cx, cy - 34.0, 10.0, Color::new(0.75, 0.7, 0.65, 1.0));
            let g = 0.6 + 0.4 * (t * 5.0).sin();
            draw_circle(cx + 26.0, cy + 6.0, 7.0, Color::new(1.0, 0.85, 0.3, g));
            draw_circle(cx + 26.0, cy + 6.0, 13.0, Color::new(1.0, 0.85, 0.3, 0.25 * g));
            draw_line(cx + 12.0, cy - 6.0, cx + 26.0, cy - 2.0, 3.0, Color::new(0.6, 0.6, 0.7, 1.0));
        }
    }

    center_at(CARD_NUMERALS[idx], cx, y + 34.0, 30, accent);
    center_at(CARD_NAMES[idx], cx, y + h - 22.0, 21, WHITE);
}

fn center_at(text: &str, cx: f32, y: f32, size: u16, color: Color) {
    let m = measure_text(text, None, size, 1.0);
    draw_text(text, cx - m.width / 2.0, y, size as f32, color);
}

/// Pantalla de bienvenida. Devuelve Some(nivel) si el jugador confirma.
pub fn draw_menu(sel: &mut usize, t: f32) -> Option<usize> {
    clear_background(Color::new(0.04, 0.03, 0.09, 1.0));
    starfield(t);

    center_text("A  R  C  A  N  A", 120.0, 72, Color::new(0.95, 0.85, 0.5, 1.0));
    center_text("un raycaster de tarot", 158.0, 26, Color::new(0.7, 0.65, 0.8, 1.0));
    center_text("Elige tu arcano", 215.0, 30, WHITE);

    let cw = 190.0;
    let ch = 300.0;
    let gap = 48.0;
    let total = cw * 3.0 + gap * 2.0;
    let x0 = (screen_width() - total) / 2.0;
    let y0 = 250.0;

    // seleccion con mouse
    let (mx, my) = mouse_position();
    for i in 0..3 {
        let x = x0 + i as f32 * (cw + gap);
        if mx >= x && mx <= x + cw && my >= y0 - 20.0 && my <= y0 + ch {
            *sel = i;
        }
        draw_tarot_card(x, y0, cw, ch, i, *sel == i, t);
    }

    center_text(
        "flechas / mouse para elegir  -  ENTER o clic para entrar",
        y0 + ch + 60.0,
        24,
        Color::new(0.8, 0.8, 0.9, 1.0),
    );
    center_text(
        "WASD moverse | mouse girar | clic izq disparar | SHIFT correr | ESC menu",
        y0 + ch + 92.0,
        20,
        Color::new(0.55, 0.55, 0.7, 1.0),
    );

    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        *sel = (*sel + 1) % 3;
    }
    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        *sel = (*sel + 2) % 3;
    }
    if is_key_pressed(KeyCode::Key1) { *sel = 0; return Some(0); }
    if is_key_pressed(KeyCode::Key2) { *sel = 1; return Some(1); }
    if is_key_pressed(KeyCode::Key3) { *sel = 2; return Some(2); }
    if is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) {
        return Some(*sel);
    }
    None
}

/// viñeta suave en los bordes para dar ambiente
pub fn draw_vignette() {
    let sw = screen_width();
    let sh = screen_height();
    for (i, a) in [(0.0f32, 0.22f32), (30.0, 0.10), (60.0, 0.05)] {
        let c = Color::new(0.0, 0.0, 0.02, a);
        draw_rectangle(0.0, i, sw, 30.0, c);
        draw_rectangle(0.0, sh - i - 30.0, sw, 30.0, c);
        draw_rectangle(i, 0.0, 30.0, sh, c);
        draw_rectangle(sw - i - 30.0, 0.0, 30.0, sh, c);
    }
}

/// HUD durante el juego
pub fn draw_hud(level_idx: usize, cards: usize, total_cards: usize, spirits_left: usize, portal_active: bool, flash: f32, bob: f32, moving: f32) {
    let sw = screen_width();
    let sh = screen_height();

    // báculo/farol del jugador con balanceo
    let sway_x = (bob).sin() * 14.0 * moving;
    let sway_y = ((bob * 2.0).cos() * 7.0 + 6.0) * moving;
    let hx = sw * 0.68 + sway_x;
    let hy = sh - 150.0 + sway_y;
    draw_line(hx + 60.0, sh + 20.0, hx, hy + 30.0, 26.0, Color::new(0.32, 0.22, 0.14, 1.0));
    draw_line(hx + 58.0, sh + 20.0, hx, hy + 30.0, 16.0, Color::new(0.45, 0.32, 0.2, 1.0));
    let accent = CARD_COLORS[level_idx];
    let g = 0.75 + 0.25 * (get_time() as f32 * 6.0).sin();
    draw_circle(hx, hy + 18.0, 14.0, Color::new(accent.r, accent.g, accent.b, g));
    draw_circle(hx, hy + 18.0, 24.0, Color::new(accent.r, accent.g, accent.b, 0.30 * g));
    if flash > 0.0 {
        draw_circle(hx, hy + 12.0, 34.0 + flash * 40.0, Color::new(1.0, 0.95, 0.6, flash * 0.9));
        draw_circle(sw / 2.0, sh / 2.0, 90.0 * flash, Color::new(1.0, 0.9, 0.5, flash * 0.25));
    }

    // mira
    let c = Color::new(1.0, 1.0, 1.0, 0.85);
    draw_line(sw / 2.0 - 11.0, sh / 2.0, sw / 2.0 - 4.0, sh / 2.0, 2.0, c);
    draw_line(sw / 2.0 + 4.0, sh / 2.0, sw / 2.0 + 11.0, sh / 2.0, 2.0, c);
    draw_line(sw / 2.0, sh / 2.0 - 11.0, sw / 2.0, sh / 2.0 - 4.0, 2.0, c);
    draw_line(sw / 2.0, sh / 2.0 + 4.0, sw / 2.0, sh / 2.0 + 11.0, 2.0, c);

    // panel inferior izquierdo: cartas recogidas
    draw_rectangle(14.0, sh - 96.0, 250.0, 82.0, Color::new(0.02, 0.02, 0.06, 0.65));
    draw_rectangle_lines(14.0, sh - 96.0, 250.0, 82.0, 2.0, Color::new(0.9, 0.8, 0.5, 0.8));
    draw_text(&format!("SELLOS  {}/{}", cards, total_cards), 26.0, sh - 72.0, 22.0, Color::new(0.9, 0.8, 0.5, 1.0));
    for i in 0..total_cards {
        let x = 26.0 + i as f32 * 40.0;
        let y = sh - 62.0;
        let got = i < cards;
        let col = if got { Color::new(1.0, 0.85, 0.25, 1.0) } else { Color::new(0.35, 0.32, 0.45, 0.8) };
        draw_rectangle(x, y, 28.0, 40.0, Color::new(0.1, 0.07, 0.18, 1.0));
        draw_rectangle_lines(x, y, 28.0, 40.0, 2.0, col);
        if got {
            draw_circle(x + 14.0, y + 20.0, 6.0, col);
        }
    }
    let msg = if portal_active {
        "Portal abierto! Entra al portal"
    } else {
        "Recoge los sellos para abrir el portal"
    };
    draw_text(msg, 150.0, sh - 40.0, 19.0, if portal_active { Color::new(0.8, 0.5, 1.0, 1.0) } else { Color::new(0.75, 0.75, 0.85, 1.0) });

    // nombre del nivel arriba a la izquierda
    draw_rectangle(14.0, 14.0, 240.0, 58.0, Color::new(0.02, 0.02, 0.06, 0.65));
    draw_rectangle_lines(14.0, 14.0, 240.0, 58.0, 2.0, CARD_COLORS[level_idx]);
    draw_text(&format!("{} - {}", CARD_NUMERALS[level_idx], CARD_NAMES[level_idx]), 26.0, 40.0, 24.0, CARD_COLORS[level_idx]);
    draw_text(&format!("Espiritus restantes: {}", spirits_left), 26.0, 62.0, 18.0, Color::new(0.7, 0.8, 1.0, 0.9));
}

/// Pantalla de éxito al superar un nivel. true = continuar.
pub fn draw_success(level_idx: usize, subtitle: &str, time_s: f32, spirits: usize, last: bool, t: f32) -> bool {
    clear_background(Color::new(0.04, 0.03, 0.09, 1.0));
    starfield(t);
    let accent = CARD_COLORS[level_idx];

    center_text("ARCANO SUPERADO", 150.0, 58, accent);
    center_text(CARD_NAMES[level_idx], 200.0, 34, WHITE);
    center_text(subtitle, 228.0, 22, Color::new(0.7, 0.68, 0.82, 1.0));

    let cw = 200.0;
    let ch = 315.0;
    draw_tarot_card((screen_width() - cw) / 2.0, 240.0, cw, ch, level_idx, true, t);

    center_text(&format!("Tiempo: {:.1} s", time_s), 600.0, 26, Color::new(0.85, 0.85, 0.95, 1.0));
    center_text(&format!("Espiritus disipados: {}", spirits), 632.0, 26, Color::new(0.7, 0.85, 1.0, 1.0));
    let hint = if last { "ESPACIO para ver tu destino" } else { "ESPACIO para el siguiente arcano  |  ESC menu" };
    let blink = 0.6 + 0.4 * (t * 3.0).sin();
    center_text(hint, 690.0, 26, Color::new(0.95, 0.85, 0.5, blink));

    is_key_pressed(KeyCode::Space)
}

/// Pantalla final tras superar los tres arcanos. true = volver al menú.
pub fn draw_victory(t: f32) -> bool {
    clear_background(Color::new(0.04, 0.03, 0.09, 1.0));
    starfield(t);
    center_text("EL VIAJE ESTA COMPLETO", 140.0, 60, Color::new(0.95, 0.85, 0.5, 1.0));
    center_text("Los tres arcanos han sido superados", 185.0, 28, WHITE);

    let cw = 170.0;
    let ch = 270.0;
    let gap = 40.0;
    let total = cw * 3.0 + gap * 2.0;
    let x0 = (screen_width() - total) / 2.0;
    for i in 0..3 {
        draw_tarot_card(x0 + i as f32 * (cw + gap), 240.0, cw, ch, i, true, t + i as f32);
    }

    let blink = 0.6 + 0.4 * (t * 3.0).sin();
    center_text("ESPACIO para volver al menu", 620.0, 28, Color::new(0.95, 0.85, 0.5, blink));
    is_key_pressed(KeyCode::Space)
}
