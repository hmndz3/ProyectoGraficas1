// Jugador: movimiento, rotación y colisión contra la cuadrícula.

use crate::level::Level;
use macroquad::prelude::*;

pub const RADIUS: f32 = 0.22;
pub const MOVE_SPEED: f32 = 3.2;
pub const RUN_SPEED: f32 = 4.8;
pub const ROT_SPEED: f32 = 2.6; // teclado
pub const MOUSE_SENS: f32 = 0.0026;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub dir: f32,
    pub bob: f32,    // fase de balanceo al caminar
    pub moving: f32, // cuanta velocidad lleva (para el bob del arma)
    pub sens: f32,   // multiplicador de sensibilidad del mouse
}

impl Player {
    pub fn new(spawn: (f32, f32), dir: f32) -> Self {
        Player {
            x: spawn.0,
            y: spawn.1,
            dir,
            bob: 0.0,
            moving: 0.0,
            sens: 1.0,
        }
    }

    pub fn dir_vec(&self) -> (f32, f32) {
        (self.dir.cos(), self.dir.sin())
    }

    /// intenta mover por ejes separados para deslizarse por las paredes
    fn try_move(&mut self, level: &Level, dx: f32, dy: f32) {
        let nx = self.x + dx;
        if !level.solid(nx + RADIUS.copysign(dx), self.y - RADIUS)
            && !level.solid(nx + RADIUS.copysign(dx), self.y + RADIUS)
        {
            self.x = nx;
        }
        let ny = self.y + dy;
        if !level.solid(self.x - RADIUS, ny + RADIUS.copysign(dy))
            && !level.solid(self.x + RADIUS, ny + RADIUS.copysign(dy))
        {
            self.y = ny;
        }
    }

    pub fn update(&mut self, level: &Level, dt: f32, mouse_dx: f32) {
        // sensibilidad ajustable con [ y ]
        if is_key_pressed(KeyCode::LeftBracket) {
            self.sens = (self.sens - 0.2).max(0.2);
        }
        if is_key_pressed(KeyCode::RightBracket) {
            self.sens = (self.sens + 0.2).min(3.0);
        }
        // rotación: mouse horizontal + flechas como respaldo
        self.dir += mouse_dx * MOUSE_SENS * self.sens;
        if is_key_down(KeyCode::Left) {
            self.dir -= ROT_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.dir += ROT_SPEED * dt;
        }

        let (fx, fy) = self.dir_vec();
        let (sx, sy) = (-fy, fx); // perpendicular (strafe)
        let mut mx = 0.0;
        let mut my = 0.0;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            mx += fx;
            my += fy;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            mx -= fx;
            my -= fy;
        }
        if is_key_down(KeyCode::A) {
            mx -= sx;
            my -= sy;
        }
        if is_key_down(KeyCode::D) {
            mx += sx;
            my += sy;
        }

        let len = (mx * mx + my * my).sqrt();
        if len > 0.001 {
            let speed = if is_key_down(KeyCode::LeftShift) {
                RUN_SPEED
            } else {
                MOVE_SPEED
            };
            self.try_move(level, mx / len * speed * dt, my / len * speed * dt);
            self.bob += dt * speed * 2.4;
            self.moving = (self.moving + dt * 6.0).min(1.0);
        } else {
            self.moving = (self.moving - dt * 6.0).max(0.0);
        }
    }
}
