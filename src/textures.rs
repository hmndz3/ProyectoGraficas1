// Texturas procedurales 64x64 — sin assets externos.

pub const TS: usize = 64; // tamaño de textura

#[derive(Clone)]
pub struct Tex {
    pub w: usize,
    pub h: usize,
    pub px: Vec<[u8; 4]>,
}

impl Tex {
    pub fn new(w: usize, h: usize) -> Self {
        Tex {
            w,
            h,
            px: vec![[0, 0, 0, 0]; w * h],
        }
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> [u8; 4] {
        self.px[(y.min(self.h - 1)) * self.w + x.min(self.w - 1)]
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, c: [u8; 4]) {
        if x < self.w && y < self.h {
            self.px[y * self.w + x] = c;
        }
    }
}

// hash determinista para ruido
fn hash(x: u32, y: u32, seed: u32) -> f32 {
    let mut h =
        x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263) ^ seed.wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    ((h ^ (h >> 16)) & 0xffff) as f32 / 65535.0
}

fn rgb(t: [f32; 3], m: f32) -> [u8; 4] {
    [
        (t[0] * m * 255.0).clamp(0.0, 255.0) as u8,
        (t[1] * m * 255.0).clamp(0.0, 255.0) as u8,
        (t[2] * m * 255.0).clamp(0.0, 255.0) as u8,
        255,
    ]
}

/// Ladrillos con mortero oscuro
pub fn brick(tint: [f32; 3], seed: u32) -> Tex {
    let mut t = Tex::new(TS, TS);
    for y in 0..TS {
        let row = y / 16;
        let off = if row % 2 == 0 { 0 } else { 16 };
        for x in 0..TS {
            let bx = (x + off) % 32;
            let mortar = y % 16 < 2 || bx < 2;
            let n = hash(((x + off) / 32) as u32, row as u32, seed) * 0.25 + 0.75;
            let g = hash(x as u32, y as u32, seed ^ 7) * 0.12;
            let m = if mortar { 0.35 } else { n - g };
            t.set(x, y, rgb(tint, m));
        }
    }
    t
}

/// Bloques de piedra grandes e irregulares
pub fn stone(tint: [f32; 3], seed: u32) -> Tex {
    let mut t = Tex::new(TS, TS);
    for y in 0..TS {
        for x in 0..TS {
            let cx = x / 21;
            let cy = y / 16;
            let n = hash(cx as u32, cy as u32, seed) * 0.3 + 0.65;
            let edge = x % 21 < 2 || y % 16 < 2;
            let g = hash(x as u32, y as u32, seed ^ 31) * 0.15;
            let m = if edge { 0.4 } else { n - g };
            t.set(x, y, rgb(tint, m));
        }
    }
    t
}

/// Piedra oscura con runas brillantes
pub fn runes(tint: [f32; 3], glow: [f32; 3], seed: u32) -> Tex {
    let mut t = stone(tint, seed);
    // glifo simple por celda de 32x32: trazos rectos pseudoaleatorios
    for gy in 0..2 {
        for gx in 0..2 {
            let ox = gx * 32 + 8;
            let oy = gy * 32 + 8;
            let s = seed ^ ((gx * 7 + gy * 13) as u32);
            let mut px = (hash(1, 1, s) * 14.0) as usize;
            let mut py = (hash(2, 2, s) * 14.0) as usize;
            for i in 0..6 {
                let nx = (hash(i + 3, 5, s) * 15.0) as usize;
                let ny = (hash(i + 9, 5, s) * 15.0) as usize;
                // línea de (px,py) a (nx,ny)
                let steps = 16;
                for k in 0..=steps {
                    let fx = px as f32 + (nx as f32 - px as f32) * k as f32 / steps as f32;
                    let fy = py as f32 + (ny as f32 - py as f32) * k as f32 / steps as f32;
                    t.set(ox + fx as usize, oy + fy as usize, rgb(glow, 1.0));
                    t.set(ox + fx as usize + 1, oy + fy as usize, rgb(glow, 0.7));
                }
                px = nx;
                py = ny;
            }
        }
    }
    t
}

/// Tablones de madera verticales
pub fn planks(tint: [f32; 3], seed: u32) -> Tex {
    let mut t = Tex::new(TS, TS);
    for y in 0..TS {
        for x in 0..TS {
            let plank = x / 8;
            let seam = x % 8 == 0;
            let grain = hash(plank as u32, (y / 3) as u32, seed) * 0.2;
            let n = hash(plank as u32, 0, seed ^ 5) * 0.2 + 0.7;
            let m = if seam { 0.3 } else { n - grain };
            t.set(x, y, rgb(tint, m));
        }
    }
    t
}

/// Estandarte de tela con emblema de estrella
pub fn banner(tint: [f32; 3], accent: [f32; 3], seed: u32) -> Tex {
    let mut t = Tex::new(TS, TS);
    for y in 0..TS {
        for x in 0..TS {
            let wave = ((y as f32 * 0.4).sin() * 1.5) as i32;
            let xi = (x as i32 + wave).rem_euclid(TS as i32) as usize;
            let border = !(6..TS - 6).contains(&xi) || !(4..TS - 4).contains(&y);
            let n = hash((x / 4) as u32, (y / 4) as u32, seed) * 0.12;
            let base = if border { 0.45 } else { 0.85 - n };
            // estrella de 4 puntas al centro
            let dx = (x as f32 - 32.0).abs();
            let dy = (y as f32 - 32.0).abs();
            let star = dx * dy < 26.0 && dx + dy < 20.0;
            if star && !border {
                t.set(x, y, rgb(accent, 1.0));
            } else {
                t.set(x, y, rgb(tint, base));
            }
        }
    }
    t
}

/// Genera el set de 5 texturas de pared para un nivel
pub fn wall_set(tints: &[[f32; 3]; 5], glow: [f32; 3], accent: [f32; 3], seed: u32) -> Vec<Tex> {
    vec![
        stone(tints[0], seed),
        brick(tints[1], seed ^ 100),
        banner(tints[2], accent, seed ^ 200),
        planks(tints[3], seed ^ 300),
        runes(tints[4], glow, seed ^ 400),
    ]
}
