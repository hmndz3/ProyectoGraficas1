// Motor de raycasting: DDA sobre la cuadrícula, columnas texturizadas,
// niebla por distancia y buffer de profundidad para los sprites.

use crate::level::Level;
use crate::textures::Tex;
use macroquad::prelude::*;

pub const RW: usize = 640; // resolución interna
pub const RH: usize = 400;
pub const FOV_PLANE: f32 = 0.72; // medio ancho del plano de cámara

pub struct Fb {
    pub img: Image,
    pub tex: Texture2D,
    pub zbuf: Vec<f32>,
}

impl Fb {
    pub fn new() -> Self {
        let img = Image::gen_image_color(RW as u16, RH as u16, BLACK);
        let tex = Texture2D::from_image(&img);
        tex.set_filter(FilterMode::Nearest);
        Fb {
            img,
            tex,
            zbuf: vec![1e9; RW],
        }
    }

    #[inline(always)]
    pub fn put(&mut self, x: usize, y: usize, c: [u8; 3]) {
        let i = (y * RW + x) * 4;
        self.img.bytes[i] = c[0];
        self.img.bytes[i + 1] = c[1];
        self.img.bytes[i + 2] = c[2];
        self.img.bytes[i + 3] = 255;
    }

    /// cielo y piso con gradiente + niebla hacia el horizonte
    pub fn clear(&mut self, ceil: [f32; 3], floor: [f32; 3], fog: [f32; 3], fog_dist: f32) {
        let half = RH / 2;
        for y in 0..RH {
            let (base, dist) = if y < half {
                let d = half as f32 / (half - y).max(1) as f32 * 0.9;
                (ceil, d)
            } else {
                let d = half as f32 / (y - half).max(1) as f32 * 0.9;
                (floor, d)
            };
            let f = (dist / fog_dist).min(1.0);
            let c = [
                ((base[0] + (fog[0] - base[0]) * f) * 255.0) as u8,
                ((base[1] + (fog[1] - base[1]) * f) * 255.0) as u8,
                ((base[2] + (fog[2] - base[2]) * f) * 255.0) as u8,
            ];
            for x in 0..RW {
                self.put(x, y, c);
            }
        }
    }

    pub fn present(&mut self) {
        self.tex.update(&self.img);
        draw_texture_ex(
            &self.tex,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}

pub struct RayHit {
    pub dist: f32,
    pub wall: u8,
    pub side: u8,
    pub wall_x: f32,
}

/// DDA clásico: lanza un rayo y devuelve la pared golpeada
pub fn cast_ray(level: &Level, px: f32, py: f32, rdx: f32, rdy: f32) -> RayHit {
    let mut map_x = px.floor() as i32;
    let mut map_y = py.floor() as i32;
    let delta_x = if rdx.abs() < 1e-8 {
        1e8
    } else {
        (1.0 / rdx).abs()
    };
    let delta_y = if rdy.abs() < 1e-8 {
        1e8
    } else {
        (1.0 / rdy).abs()
    };
    let (step_x, mut side_x) = if rdx < 0.0 {
        (-1, (px - map_x as f32) * delta_x)
    } else {
        (1, (map_x as f32 + 1.0 - px) * delta_x)
    };
    let (step_y, mut side_y) = if rdy < 0.0 {
        (-1, (py - map_y as f32) * delta_y)
    } else {
        (1, (map_y as f32 + 1.0 - py) * delta_y)
    };

    let mut side = 0u8;
    let mut wall = 1u8;
    for _ in 0..256 {
        if side_x < side_y {
            side_x += delta_x;
            map_x += step_x;
            side = 0;
        } else {
            side_y += delta_y;
            map_y += step_y;
            side = 1;
        }
        wall = level.cell(map_x, map_y);
        if wall != 0 {
            break;
        }
    }

    let dist = if side == 0 {
        side_x - delta_x
    } else {
        side_y - delta_y
    };
    let dist = dist.max(0.0001);
    let hit = if side == 0 {
        py + dist * rdy
    } else {
        px + dist * rdx
    };
    RayHit {
        dist,
        wall,
        side,
        wall_x: hit - hit.floor(),
    }
}

/// Renderiza todas las columnas de pared en el framebuffer
pub fn render_walls(
    fb: &mut Fb,
    level: &Level,
    walls: &[Tex],
    px: f32,
    py: f32,
    dir: f32,
    fog: [f32; 3],
    fog_dist: f32,
) {
    let (dx, dy) = (dir.cos(), dir.sin());
    let (plx, ply) = (-dy * FOV_PLANE, dx * FOV_PLANE);

    for x in 0..RW {
        let cam = 2.0 * x as f32 / RW as f32 - 1.0;
        let rdx = dx + plx * cam;
        let rdy = dy + ply * cam;
        let hit = cast_ray(level, px, py, rdx, rdy);
        fb.zbuf[x] = hit.dist;

        let line_h = (RH as f32 / hit.dist) as i32;
        let start = ((RH as i32 - line_h) / 2).max(0);
        let end = ((RH as i32 + line_h) / 2).min(RH as i32 - 1);

        let tex = &walls[(hit.wall.max(1) as usize - 1).min(walls.len() - 1)];
        let tx = ((hit.wall_x * tex.w as f32) as usize).min(tex.w - 1);
        // sombreado: caras Y mas oscuras + niebla por distancia
        let shade = if hit.side == 1 { 0.72 } else { 1.0 };
        let f = (hit.dist / fog_dist).min(1.0);
        let fr = fog[0] * 255.0;
        let fg = fog[1] * 255.0;
        let fbl = fog[2] * 255.0;

        let step = tex.h as f32 / line_h.max(1) as f32;
        let mut tpos = (start - (RH as i32 - line_h) / 2) as f32 * step;
        for y in start..=end {
            let ty = (tpos as usize).min(tex.h - 1);
            tpos += step;
            let c = tex.get(tx, ty);
            let r = c[0] as f32 * shade;
            let g = c[1] as f32 * shade;
            let b = c[2] as f32 * shade;
            fb.put(
                x,
                y as usize,
                [
                    (r + (fr - r) * f) as u8,
                    (g + (fg - g) * f) as u8,
                    (b + (fbl - b) * f) as u8,
                ],
            );
        }
    }
}
