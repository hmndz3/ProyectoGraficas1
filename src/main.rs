// ARCANA — un raycaster de tarot.
// Tres arcanos, tres niveles: The Fool, The Hanged Man, The Hermit.

mod audio;
mod level;
mod minimap;
mod player;
mod raycast;
mod sprites;
mod textures;
mod ui;

use audio::Audio;
use level::Level;
use macroquad::prelude::*;
use player::Player;
use raycast::Fb;
use sprites::{Kind, Sprite};
use textures::Tex;

enum State {
    Menu,
    Playing,
    Success { time_s: f32, dispelled: usize },
    Victory,
}

struct World {
    level_idx: usize,
    level: Level,
    player: Player,
    sprites: Vec<Sprite>,
    walls: Vec<Tex>,
    card_tex: Vec<Tex>,
    spirit_tex: Vec<Tex>,
    portal_tex: Vec<Tex>,
    cards_total: usize,
    cards_got: usize,
    dispelled: usize,
    portal_active: bool,
    start_time: f64,
    flash: f32,
    cooldown: f32,
}

impl World {
    fn new(idx: usize, def: &level::LevelDef) -> World {
        let level = Level::parse(def);
        let player = Player::new(level.spawn, level.spawn_dir);
        let mut sprites = Vec::new();
        for (i, &(x, y)) in level.cards.iter().enumerate() {
            sprites.push(Sprite { x, y, kind: Kind::Card, alive: true, phase: i as f32 * 1.7 });
        }
        for (i, &(x, y)) in level.spirits.iter().enumerate() {
            sprites.push(Sprite { x, y, kind: Kind::Spirit, alive: true, phase: i as f32 * 2.3 + 0.5 });
        }
        sprites.push(Sprite { x: level.portal.0, y: level.portal.1, kind: Kind::Portal, alive: true, phase: 0.0 });

        let cards_total = level.cards.len();
        World {
            level_idx: idx,
            level,
            player,
            sprites,
            walls: textures::wall_set(&def.wall_tints, def.glow, def.accent, def.seed),
            card_tex: sprites::card_frames(def.glow),
            spirit_tex: sprites::spirit_frames(),
            portal_tex: sprites::portal_frames(def.glow),
            cards_total,
            cards_got: 0,
            dispelled: 0,
            portal_active: false,
            start_time: get_time(),
            flash: 0.0,
            cooldown: 0.0,
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "ARCANA - raycaster de tarot".to_owned(),
        window_width: 1280,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let defs = level::all_levels();
    let mut audio = Audio::load().await;
    let mut fb = Fb::new();

    let mut state = State::Menu;
    let mut sel = 0usize;
    let mut world: Option<World> = None;
    let mut grabbed = false;
    let mut last_mouse: Vec2 = mouse_position().into();

    let mut fullscreen = false;

    loop {
        let t = get_time() as f32;
        let dt = get_frame_time().min(0.05);

        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }

        // captura del mouse solo dentro del juego
        let want_grab = matches!(state, State::Playing);
        if want_grab != grabbed {
            grabbed = want_grab;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
            last_mouse = mouse_position().into();
        }

        match state {
            State::Menu => {
                audio.stop_music();
                if let Some(pick) = ui::draw_menu(&mut sel, t) {
                    let idx = pick.min(defs.len() - 1);
                    world = Some(World::new(idx, defs[idx]));
                    audio.play_music(idx);
                    audio.sfx(&audio.portal);
                    state = State::Playing;
                }
            }
            State::Playing => {
                let w = world.as_mut().unwrap();
                let def = defs[w.level_idx];

                // delta de mouse para rotacion horizontal
                let mp: Vec2 = mouse_position().into();
                let mdx = mp.x - last_mouse.x;
                last_mouse = mp;

                w.player.update(&w.level, dt, mdx);
                sprites::update_spirits(&mut w.sprites, &w.level, w.player.x, w.player.y, t, dt);
                w.flash = (w.flash - dt * 5.0).max(0.0);
                w.cooldown = (w.cooldown - dt).max(0.0);

                // disparo
                if is_mouse_button_pressed(MouseButton::Left) && w.cooldown <= 0.0 {
                    w.cooldown = 0.32;
                    w.flash = 1.0;
                    audio.sfx(&audio.shoot);
                    let (fx, fy) = w.player.dir_vec();
                    let mut best: Option<(usize, f32)> = None;
                    for (i, s) in w.sprites.iter().enumerate() {
                        if !s.alive || s.kind != Kind::Spirit {
                            continue;
                        }
                        let dx = s.x - w.player.x;
                        let dy = s.y - w.player.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist > 14.0 || dist < 0.2 {
                            continue;
                        }
                        let along = dx * fx + dy * fy;
                        if along <= 0.0 {
                            continue;
                        }
                        let perp = (dx * -fy + dy * fx).abs();
                        if perp > 0.42 {
                            continue;
                        }
                        let hit = raycast::cast_ray(&w.level, w.player.x, w.player.y, dx / dist, dy / dist);
                        if hit.dist + 0.2 < dist {
                            continue; // pared en medio
                        }
                        if best.map_or(true, |(_, bd)| dist < bd) {
                            best = Some((i, dist));
                        }
                    }
                    if let Some((i, _)) = best {
                        w.sprites[i].alive = false;
                        w.dispelled += 1;
                        audio.sfx(&audio.poof);
                    }
                }

                // recoger cartas
                for s in w.sprites.iter_mut() {
                    if s.alive && s.kind == Kind::Card {
                        let d2 = (s.x - w.player.x).powi(2) + (s.y - w.player.y).powi(2);
                        if d2 < 0.42 {
                            s.alive = false;
                            w.cards_got += 1;
                            audio.sfx(&audio.pickup);
                            if w.cards_got == w.cards_total {
                                w.portal_active = true;
                                audio.sfx(&audio.portal);
                            }
                        }
                    }
                }

                // entrar al portal
                if w.portal_active {
                    let d2 = (w.level.portal.0 - w.player.x).powi(2) + (w.level.portal.1 - w.player.y).powi(2);
                    if d2 < 0.5 {
                        audio.sfx(&audio.win);
                        state = State::Success {
                            time_s: (get_time() - w.start_time) as f32,
                            dispelled: w.dispelled,
                        };
                    }
                }

                // render 3D
                fb.clear(def.ceil, def.floor, def.fog, def.fog_dist);
                raycast::render_walls(
                    &mut fb, &w.level, &w.walls,
                    w.player.x, w.player.y, w.player.dir,
                    def.fog, def.fog_dist,
                );
                sprites::render_sprites(
                    &mut fb, &w.sprites,
                    &w.card_tex, &w.spirit_tex, &w.portal_tex,
                    w.player.x, w.player.y, w.player.dir, t,
                    def.fog, def.fog_dist, w.portal_active,
                );
                fb.present();

                // HUD + minimapa
                let spirits_left = w.sprites.iter().filter(|s| s.alive && s.kind == Kind::Spirit).count();
                ui::draw_hud(
                    w.level_idx, w.cards_got, w.cards_total, spirits_left,
                    w.portal_active, w.flash, w.player.bob, w.player.moving,
                );
                minimap::draw_minimap(&w.level, &w.player, &w.sprites, w.portal_active);
                draw_text(&format!("FPS {}", get_fps()), 20.0, screen_height() - 12.0, 18.0, Color::new(1.0, 1.0, 1.0, 0.5));

                if is_key_pressed(KeyCode::Escape) {
                    state = State::Menu;
                }
            }
            State::Success { time_s, dispelled } => {
                let idx = world.as_ref().unwrap().level_idx;
                let last = idx == defs.len() - 1;
                if ui::draw_success(idx, defs[idx].subtitle, time_s, dispelled, last, t) {
                    if last {
                        audio.stop_music();
                        state = State::Victory;
                    } else {
                        let next = idx + 1;
                        world = Some(World::new(next, defs[next]));
                        audio.play_music(next);
                        state = State::Playing;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    state = State::Menu;
                }
            }
            State::Victory => {
                if ui::draw_victory(t) {
                    state = State::Menu;
                }
            }
        }

        next_frame().await;
    }
}
