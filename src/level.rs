// Definición de niveles: los tres arcanos del tarot.

pub struct LevelDef {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub grid: &'static [&'static str],
    pub ceil: [f32; 3],
    pub floor: [f32; 3],
    pub fog: [f32; 3],
    pub fog_dist: f32,
    pub wall_tints: [[f32; 3]; 5],
    pub glow: [f32; 3],
    pub accent: [f32; 3],
    pub seed: u32,
}

pub struct Level {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<u8>, // 0 = libre, 1..=5 tipo de pared
    pub spawn: (f32, f32),
    pub spawn_dir: f32,
    pub cards: Vec<(f32, f32)>,
    pub spirits: Vec<(f32, f32)>,
    pub portal: (f32, f32),
}

impl Level {
    pub fn parse(def: &LevelDef) -> Level {
        let h = def.grid.len();
        let w = def.grid[0].len();
        let mut cells = vec![0u8; w * h];
        let mut spawn = (1.5, 1.5);
        let mut cards = Vec::new();
        let mut spirits = Vec::new();
        let mut portal = (1.5, 1.5);
        for (y, row) in def.grid.iter().enumerate() {
            assert_eq!(
                row.len(),
                w,
                "fila {} de '{}' con largo distinto",
                y,
                def.name
            );
            for (x, ch) in row.bytes().enumerate() {
                let cx = x as f32 + 0.5;
                let cy = y as f32 + 0.5;
                match ch {
                    b'1'..=b'5' => cells[y * w + x] = ch - b'0',
                    b'P' => spawn = (cx, cy),
                    b'C' => cards.push((cx, cy)),
                    b'S' => spirits.push((cx, cy)),
                    b'X' => portal = (cx, cy),
                    _ => {}
                }
            }
        }
        Level {
            w,
            h,
            cells,
            spawn,
            spawn_dir: 0.0,
            cards,
            spirits,
            portal,
        }
    }

    #[inline(always)]
    pub fn cell(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 {
            return 1;
        }
        self.cells[y as usize * self.w + x as usize]
    }

    #[inline(always)]
    pub fn solid(&self, x: f32, y: f32) -> bool {
        self.cell(x.floor() as i32, y.floor() as i32) != 0
    }
}

// ---------------------------------------------------------------
// NIVEL I — THE FOOL (El Loco): acantilado luminoso, cielo abierto
// ---------------------------------------------------------------
pub const LEVEL_FOOL: LevelDef = LevelDef {
    name: "THE FOOL",
    subtitle: "0 - El comienzo del viaje",
    grid: &[
        "11111111111111111111",
        "1....S...1.......C.1",
        "1.22222..1..33333..1",
        "1.2...2..1..3...3..1",
        "1.2.C.2.....3...3..1",
        "1.2...2..1..33.33..1",
        "1.22.22..1......S..1",
        "1......S.1..44444..1",
        "1.55555..1..4...4..1",
        "1.5...5..1..4.X.4..1",
        "1.5.C.5.....4...4..1",
        "1.5...5..1..44.44..1",
        "1.55.55..1......S..1",
        "1P.......1.........1",
        "11111111111111111111",
    ],
    ceil: [0.50, 0.72, 0.95],
    floor: [0.82, 0.74, 0.55],
    fog: [0.80, 0.85, 0.97],
    fog_dist: 18.0,
    wall_tints: [
        [0.95, 0.92, 0.85], // 1 piedra clara del acantilado
        [0.85, 0.55, 0.35], // 2 ladrillo terracota
        [0.95, 0.85, 0.40], // 3 estandarte dorado
        [0.75, 0.58, 0.38], // 4 madera calida
        [0.55, 0.50, 0.42], // 5 piedra con runas
    ],
    glow: [1.0, 0.85, 0.3],
    accent: [1.0, 1.0, 1.0],
    seed: 11,
};

// ------------------------------------------------------------------
// NIVEL II — THE HANGED MAN (El Colgado): anillos suspendidos, mundo
// invertido: el techo es abismo oscuro y el piso brilla pálido.
// ------------------------------------------------------------------
pub const LEVEL_HANGED: LevelDef = LevelDef {
    name: "THE HANGED MAN",
    subtitle: "XII - La perspectiva invertida",
    grid: &[
        "11111111111111111111",
        "1P...............S.1",
        "1.2222222222222222.1",
        "1.2.............C2.1",
        "1.2.33333.333333.2.1",
        "1.2.3....S.....3.2.1",
        "1.2.3.44444444.3.2.1",
        "1.2.3.4.C.X..4.3.2.1",
        "1.2.3.44444.44.3.2.1",
        "1.2.3..S.......3.2.1",
        "1.2.333333333333.2.1",
        "1.2......C.......2.1",
        "1.2222222.22222222.1",
        "1..S...............1",
        "11111111111111111111",
    ],
    ceil: [0.05, 0.03, 0.11],
    floor: [0.48, 0.45, 0.58],
    fog: [0.13, 0.08, 0.22],
    fog_dist: 13.0,
    wall_tints: [
        [0.42, 0.40, 0.55], // 1 pizarra
        [0.22, 0.58, 0.58], // 2 ladrillo verdemar
        [0.52, 0.30, 0.68], // 3 estandarte violeta
        [0.34, 0.26, 0.42], // 4 madera fria
        [0.30, 0.30, 0.45], // 5 runas
    ],
    glow: [0.35, 0.95, 0.90],
    accent: [0.85, 0.95, 1.00],
    seed: 22,
};

// ------------------------------------------------------------------
// NIVEL III — THE HERMIT (El Ermitaño): criptas en penumbra, la niebla
// corta obliga a avanzar a la luz del farol.
// ------------------------------------------------------------------
pub const LEVEL_HERMIT: LevelDef = LevelDef {
    name: "THE HERMIT",
    subtitle: "IX - La luz interior",
    grid: &[
        "11111111111111111111",
        "1P.....1.....S.....1",
        "1.5555.1.55555555..1",
        "1.5..5.1.5......5..1",
        "1.5C.5.....S....5..1",
        "1.5..5.1.5..C...5..1",
        "1.55.5.1.55555.55..1",
        "1..................1",
        "1.22222.111.22222..1",
        "1.2...2..1..2...2..1",
        "1.2.S.2..1..2.X.2..1",
        "1.2...2..1..2...2..1",
        "1.22.22..1..22.22..1",
        "1........1.......C.1",
        "11111111111111111111",
    ],
    ceil: [0.02, 0.02, 0.05],
    floor: [0.08, 0.08, 0.11],
    fog: [0.01, 0.01, 0.03],
    fog_dist: 8.0,
    wall_tints: [
        [0.36, 0.37, 0.44], // 1 piedra de cripta
        [0.30, 0.33, 0.46], // 2 ladrillo helado
        [0.42, 0.44, 0.58], // 3 estandarte gris
        [0.30, 0.25, 0.20], // 4 madera vieja
        [0.28, 0.28, 0.36], // 5 runas
    ],
    glow: [1.00, 0.80, 0.35],
    accent: [0.92, 0.88, 0.70],
    seed: 33,
};

pub fn all_levels() -> Vec<&'static LevelDef> {
    vec![&LEVEL_FOOL, &LEVEL_HANGED, &LEVEL_HERMIT]
}

#[cfg(test)]
mod tests {
    use super::*;

    // BFS desde el spawn: toda carta, espíritu y el portal deben ser alcanzables
    fn check_reachable(def: &LevelDef) {
        let lv = Level::parse(def);
        let mut vis = vec![false; lv.w * lv.h];
        let start = (lv.spawn.0 as usize, lv.spawn.1 as usize);
        let mut q = std::collections::VecDeque::new();
        q.push_back(start);
        vis[start.1 * lv.w + start.0] = true;
        while let Some((x, y)) = q.pop_front() {
            for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if lv.cell(nx, ny) == 0 && !vis[ny as usize * lv.w + nx as usize] {
                    vis[ny as usize * lv.w + nx as usize] = true;
                    q.push_back((nx as usize, ny as usize));
                }
            }
        }
        let reach = |p: (f32, f32)| vis[p.1 as usize * lv.w + p.0 as usize];
        assert_eq!(lv.cards.len(), 3, "{}: deben ser 3 cartas", def.name);
        for (i, c) in lv.cards.iter().enumerate() {
            assert!(
                reach(*c),
                "{}: carta {} inalcanzable en {:?}",
                def.name,
                i,
                c
            );
        }
        for (i, s) in lv.spirits.iter().enumerate() {
            assert!(
                reach(*s),
                "{}: espiritu {} inalcanzable en {:?}",
                def.name,
                i,
                s
            );
        }
        assert!(reach(lv.portal), "{}: portal inalcanzable", def.name);
    }

    #[test]
    fn niveles_jugables() {
        for def in all_levels() {
            check_reachable(def);
        }
    }
}
