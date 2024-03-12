extern crate coffee;
use coffee::graphics::{
    Color, Frame, Mesh, Rectangle, Shape
};
use coffee::input::keyboard::KeyCode;

use crate::Position;

pub struct Player {
    pub position: Vec<Position>,
    pub direction: Option<KeyCode>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: vec![],
            direction: None,
        }
    }

    pub fn create_player(&mut self) {
        self.position.push(Position(20.0, 45.0));
    }

    pub fn draw_player(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        let mut index = 0;
        for pos in &self.position {
            index += 1;
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: pos.0,
                    y: pos.1,
                    width: 50.0,
                    height: 50.0,
                }),
                Color::GREEN,
            );
            if index >= 2  {
                break
            }
        }
        mesh.draw(&mut frame.as_target());
    }
    
    pub fn move_right(&mut self) {
        let player = self.position.last().unwrap().clone();
        println!("{}", player.0);
        if player.0 <= 880.0 {
            self.position.push(Position(player.0 + 7.0, player.1));
        } else {
            self.position.push(Position(0.0, player.1));
        }
    }

    pub fn move_left(&mut self) {
        let player = self.position.last().unwrap().clone();
        if player.0 >= 0.0 {
            self.position.push(Position(player.0 - 7.0, player.1));
        } else {
            self.position.push(Position(870.0, player.1));
        }
    }

    pub fn move_down(&mut self) {        
        let player = self.position.last().unwrap().clone();
        if player.1 <= 800.0 {
            self.position.push(Position(player.0, player.1 + 10.0));
        } else {
            self.position.push(Position(player.0, 0.0));
        }
    }

    pub fn move_up(&mut self) {
        let player = self.position.last().unwrap().clone();
        if player.1 != 0.0 {
            self.position.push(Position(player.0, player.1 - 10.0));
        } else {        
            self.position.push(Position(player.0, 570.0));
        }
    }

    pub fn stop(&mut self) {
        let head = self.position.last().unwrap().clone();
        if head.1 != 0.0 {
            self.position.push(Position(head.0, head.1));
        } else {        
            self.position.push(Position(head.0, 570.0));
        }
    }

    pub fn move_to(&mut self, keycode: Option<KeyCode>) {
        self.direction = keycode;
        match keycode {
            Some(KeyCode::W) => {
                self.move_up();
            }
            Some(KeyCode::A) => {
                self.move_left();
            }
            Some(KeyCode::S) => {
                self.move_down();
            }
            Some(KeyCode::D) => {
                self.move_right();
            }
            _ => {
                self.stop();
            },
        }    
    }

    pub fn died(&self) -> bool {
        for i in &self.position[..self.position.len() - 1] {
            if *i == *self.position.last().unwrap() {
                return true;
            }
        }
        false
    }
}


pub struct Enemy {
    pub position: Vec<Position>,
    pub direction: Option<KeyCode>,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            position: vec![],
            direction: None,
        }
    }

    pub fn create_enemy(&mut self) {
        self.position.push(Position(150.0, 45.0));
    }

    pub fn draw_enemy(&mut self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        let mut index = 0;
        for pos in &self.position {
            index += 1;
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: pos.0,
                    y: pos.1,
                    width: 10.0,
                    height: 10.0,
                }),
                Color::RED,
            );
            if index >= 2  {
                break
            }
        }
        mesh.draw(&mut frame.as_target());
    }
    
    pub fn move_right(&mut self) {
        let head = self.position.last().unwrap().clone();
        if head.0 != 870.0 {
            self.position.push(Position(head.0 + 10.0, head.1));
        } else {
            self.position.push(Position(0.0, head.1));
        }
    }

    pub fn move_left(&mut self) {
        let enemy = self.position.last().unwrap().clone();
        if enemy.0 != 0.0 {
            self.position.push(Position(enemy.0 - 10.0, enemy.1));
        } else {
            self.position.push(Position(870.0, enemy.1));
        }
    }

    pub fn move_down(&mut self) {        
        let enemy = self.position.last().unwrap().clone();
        if enemy.1 <= 800.0 {
            self.position.push(Position(enemy.0, enemy.1 + 15.0));
        } else {
            self.position.push(Position(enemy.0, 0.0));
        }
    }

    pub fn move_up(&mut self) {
        let enemy = self.position.last().unwrap().clone();
        if enemy.1 != 0.0 {
            self.position.push(Position(enemy.0, enemy.1 - 15.0));
        } else {        
            self.position.push(Position(enemy.0, 570.0));
        }
    }

    pub fn stop(&mut self) {
        let enemy = self.position.last().unwrap().clone();
        if enemy.1 != 0.0 {
            self.position.push(Position(enemy.0, enemy.1));
        } else {        
            self.position.push(Position(enemy.0, 570.0));
        }
    }

    pub fn move_to(&mut self, keycode: Option<KeyCode>) {
        self.direction = keycode;
        match keycode {
            Some(KeyCode::Up) => {
                self.move_up();
            }
            Some(KeyCode::Left) => {
                self.move_left();
            }
            Some(KeyCode::Down) => {
                self.move_down();
            }
            Some(KeyCode::Right) => {
                self.move_right();
            }
            _ => {
                self.stop();
            },
        }    
    }

    pub fn died(&self) -> bool {
        for i in &self.position[..self.position.len() - 1] {
            if *i == *self.position.last().unwrap() {
                return true;
            }
        }
        false
    }
}

pub fn input(key: KeyCode) -> Option<KeyCode> {
    match key {
        KeyCode::W => {
            return Some(key);
        }
        KeyCode::A => {
            return Some(key);
        }
        KeyCode::S => {
            return Some(key);
        }
        KeyCode::D => {
            return Some(key);
        }
        KeyCode::Up => {
            return Some(key);
        }
        KeyCode::Left => {
            return Some(key);
        }
        KeyCode::Down => {
            return Some(key);
        }
        KeyCode::Right => {
            return Some(key);
        }
        _ => { 
            return None; 
        },
    }
}
