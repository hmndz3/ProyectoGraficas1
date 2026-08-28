// Sprites tipo billboard: cartas del tarot (coleccionables), espíritus
// errantes (se pueden disipar disparando) y el portal de salida.
// Todos con animación por cuadros generada proceduralmente.

use crate::level::Level;
use crate::raycast::{Fb, FOV_PLANE, RH, RW};
use crate::textures::{Tex, TS};

pub const FRAMES: usize = 4;

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Card,
    Spirit,
    Portal,
}

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub kind: Kind,
    pub alive: bool,
    pub phase: f32, // desfase de animación
}

fn set_a(t: &mut Tex, x: i32, y: i32, c: [u8; 4]) {
    if x >= 0 && y >= 0 {
        t.set(x as usize, y as usize, c);
    }
}

/// Carta de tarot flotante con halo pulsante (4 cuadros)
pub fn card_frames(accent: [f32; 3]) -> Vec<Tex> {
    let mut out = Vec::new();
    for f in 0..FRAMES {
        let mut t = Tex::new(TS, TS);
        let pulse = 0.55 + 0.45 * (f as f32 / FRAMES as f32 * std::f32::consts::TAU).sin();
        let bob = ((f as f32 / FRAMES as f32 * std::f32::consts::TAU).sin() * 3.0) as i32;
        // halo
        for y in 0..TS as i32 {
            for x in 0..TS as i32 {
                let dx = x - 32;
                let dy = y - 30 - bob;
                let d = ((dx * dx + dy * dy) as f32).sqrt();
                if d > 24.0 && d < 30.0 {
                    let a = ((30.0 - d) / 6.0 * 120.0 * pulse) as u8;
                    set_a(
                        &mut t,
                        x,
                        y,
                        [
                            (accent[0] * 255.0) as u8,
                            (accent[1] * 255.0) as u8,
                            (accent[2] * 255.0) as u8,
                            a,
                        ],
                    );
                }
            }
        }
        // cuerpo de la carta (18..46 x, 9..53 y) con borde dorado
        for y in 9..53 {
            for x in 18..46 {
                let yy = y + bob;
                let border = !(20..44).contains(&x) || !(11..51).contains(&y);
                let c = if border {
                    [235, 200, 90, 255]
                } else {
                    [48, 28, 78, 255] // reverso violeta
                };
                set_a(&mut t, x, yy, c);
            }
        }
        // estrella central
        for y in 0..TS as i32 {
            for x in 0..TS as i32 {
                let dx = (x - 32).abs();
                let dy = (y - 31 - bob).abs();
                if dx * dy < 14 && dx + dy < 11 {
                    set_a(&mut t, x, y, [255, 235, 150, 255]);
                }
            }
        }
        out.push(t);
    }
    out
}

/// Espíritu fantasmal con borde inferior ondulante (4 cuadros)
pub fn spirit_frames() -> Vec<Tex> {
    let mut out = Vec::new();
    for f in 0..FRAMES {
        let mut t = Tex::new(TS, TS);
        let ph = f as f32 / FRAMES as f32 * std::f32::consts::TAU;
        for x in 12..52i32 {
            // cabeza redonda
            let dx = x - 32;
            let head_half = {
                let v = 20 * 20 - dx * dx;
                if v > 0 {
                    (v as f32).sqrt() as i32
                } else {
                    -1
                }
            };
            if head_half >= 0 {
                let top = 26 - head_half;
                let bottom = 44 + ((x as f32 * 0.55 + ph).sin() * 4.0) as i32;
                for y in top..bottom.min(60) {
                    let a = if y < 30 { 215 } else { 215 - (y - 30) * 5 };
                    set_a(&mut t, x, y, [190, 220, 255, a.clamp(40, 255) as u8]);
                }
            }
        }
        // ojos
        for (ex, ey) in [(25, 24), (39, 24)] {
            for y in -2..3i32 {
                for x in -2..3i32 {
                    if x * x + y * y <= 4 {
                        set_a(&mut t, ex + x, ey + y, [20, 20, 60, 255]);
                    }
                }
            }
        }
        out.push(t);
    }
    out
}

/// Portal: anillo de orbes girando (4 cuadros)
pub fn portal_frames(color: [f32; 3]) -> Vec<Tex> {
    let mut out = Vec::new();
    for f in 0..FRAMES {
        let mut t = Tex::new(TS, TS);
        let rot = f as f32 / FRAMES as f32 * std::f32::consts::TAU / 6.0;
        // neblina interior
        for y in 0..TS as i32 {
            for x in 0..TS as i32 {
                let dx = (x - 32) as f32;
                let dy = (y - 32) as f32;
                let d = (dx * dx + dy * dy).sqrt();
                if d < 22.0 {
                    let a = ((22.0 - d) / 22.0 * 90.0) as u8;
                    set_a(
                        &mut t,
                        x,
                        y,
                        [
                            (color[0] * 160.0) as u8,
                            (color[1] * 160.0) as u8,
                            (color[2] * 160.0) as u8,
                            a,
                        ],
                    );
                }
            }
        }
        // orbes del anillo
        for i in 0..12 {
            let a = i as f32 / 12.0 * std::f32::consts::TAU + rot;
            let ox = 32.0 + a.cos() * 26.0;
            let oy = 32.0 + a.sin() * 26.0;
            for y in -3..4i32 {
                for x in -3..4i32 {
                    let d2 = x * x + y * y;
                    if d2 <= 9 {
                        let al = if d2 <= 4 { 255 } else { 140 };
                        set_a(
                            &mut t,
                            ox as i32 + x,
                            oy as i32 + y,
                            [
                                (color[0] * 255.0) as u8,
                                (color[1] * 255.0) as u8,
                                (color[2] * 255.0) as u8,
                                al,
                            ],
                        );
                    }
                }
            }
        }
        out.push(t);
    }
    out
}

/// Los espíritus vagan y se acercan lentamente al jugador
pub fn update_spirits(sprites: &mut [Sprite], level: &Level, px: f32, py: f32, t: f32, dt: f32) {
    for s in sprites.iter_mut() {
        if !s.alive || s.kind != Kind::Spirit {
            continue;
        }
        let dx = px - s.x;
        let dy = py - s.y;
        let d = (dx * dx + dy * dy).sqrt().max(0.001);
        let (mut vx, mut vy) = if d < 7.0 && d > 1.2 {
            (dx / d, dy / d)
        } else {
            ((t * 0.7 + s.phase).cos(), (t * 0.9 + s.phase * 2.0).sin())
        };
        vx *= 0.55 * dt;
        vy *= 0.55 * dt;
        if !level.solid(s.x + vx, s.y) {
            s.x += vx;
        }
        if !level.solid(s.x, s.y + vy) {
            s.y += vy;
        }
    }
}

/// Dibuja los sprites como billboards respetando el z-buffer
#[allow(clippy::too_many_arguments)]
pub fn render_sprites(
    fb: &mut Fb,
    sprites: &[Sprite],
    card_tex: &[Tex],
    spirit_tex: &[Tex],
    portal_tex: &[Tex],
    px: f32,
    py: f32,
    dir: f32,
    t: f32,
    fog: [f32; 3],
    fog_dist: f32,
    portal_active: bool,
) {
    let (dx, dy) = (dir.cos(), dir.sin());
    let (plx, ply) = (-dy * FOV_PLANE, dx * FOV_PLANE);
    let inv_det = 1.0 / (plx * dy - dx * ply);

    // ordenar lejano -> cercano
    let mut order: Vec<usize> = (0..sprites.len()).filter(|&i| sprites[i].alive).collect();
    order.sort_by(|&a, &b| {
        let da = (sprites[a].x - px).powi(2) + (sprites[a].y - py).powi(2);
        let db = (sprites[b].x - px).powi(2) + (sprites[b].y - py).powi(2);
        db.partial_cmp(&da).unwrap()
    });

    for i in order {
        let s = &sprites[i];
        let sx = s.x - px;
        let sy = s.y - py;
        let tr_x = inv_det * (dy * sx - dx * sy);
        let tr_y = inv_det * (-ply * sx + plx * sy);
        if tr_y <= 0.1 {
            continue;
        }
        let screen_x = ((RW as f32 / 2.0) * (1.0 + tr_x / tr_y)) as i32;
        let size = (RH as f32 / tr_y) as i32;
        if size <= 1 {
            continue;
        }

        let frames = match s.kind {
            Kind::Card => card_tex,
            Kind::Spirit => spirit_tex,
            Kind::Portal => portal_tex,
        };
        let fi = ((t * 6.0 + s.phase) as usize) % FRAMES;
        let tex = &frames[fi];

        // tinte: portal inactivo se ve apagado
        let tint = if s.kind == Kind::Portal && !portal_active {
            0.35
        } else {
            1.0
        };
        let fogf = (tr_y / fog_dist).min(1.0);
        let fr = fog[0] * 255.0;
        let fg = fog[1] * 255.0;
        let fbl = fog[2] * 255.0;

        let x0 = (screen_x - size / 2).max(0);
        let x1 = (screen_x + size / 2).min(RW as i32 - 1);
        let y0 = ((RH as i32 - size) / 2).max(0);
        let y1 = ((RH as i32 + size) / 2).min(RH as i32 - 1);

        for x in x0..=x1 {
            if tr_y >= fb.zbuf[x as usize] {
                continue;
            }
            let tx = ((x - (screen_x - size / 2)) * TS as i32 / size).clamp(0, TS as i32 - 1);
            for y in y0..=y1 {
                let ty = ((y - (RH as i32 - size) / 2) * TS as i32 / size).clamp(0, TS as i32 - 1);
                let c = tex.get(tx as usize, ty as usize);
                if c[3] < 40 {
                    continue;
                }
                let a = c[3] as f32 / 255.0;
                let i4 = (y as usize * RW + x as usize) * 4;
                let br = fb.img.bytes[i4] as f32;
                let bg = fb.img.bytes[i4 + 1] as f32;
                let bb = fb.img.bytes[i4 + 2] as f32;
                let mut r = c[0] as f32 * tint;
                let mut g = c[1] as f32 * tint;
                let mut b = c[2] as f32 * tint;
                r += (fr - r) * fogf;
                g += (fg - g) * fogf;
                b += (fbl - b) * fogf;
                fb.img.bytes[i4] = (br + (r - br) * a) as u8;
                fb.img.bytes[i4 + 1] = (bg + (g - bg) * a) as u8;
                fb.img.bytes[i4 + 2] = (bb + (b - bb) * a) as u8;
            }
        }
    }
}
