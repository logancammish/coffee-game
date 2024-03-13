extern crate coffee;
use std::thread;
use std::time::Duration;

use coffee::graphics::{
    Color, Font, Frame, Point, Text, Window, WindowSettings
};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};
use coffee::load::Task;
use coffee::{Game, Timer};
mod terrain;
use terrain::generation;
mod ai; 
use ai::{Player, Enemy};
use console::Term;

fn main() {
    FFGame::run(WindowSettings {
        title: String::from("Logan Cammish Game"),
        size: (900, 800),
        resizable: false,
        maximized: false,
        fullscreen: false,
    }).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(f32, f32);

struct FFGame {
    player: Player,
    enemy: Enemy,

    ticks: u64,    
    lkey: Option<KeyCode>,

    phealth: u8,
    ehealth: u8,
    pwins: u64, 
    ewins: u64, 
}
pub enum ButtonState {
    PlayAgain,
}
impl Game for FFGame {
    const TICKS_PER_SECOND: u16 = 60;
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<FFGame> {
        let mut player = Player::new();
        player.create_player();
        let mut enemy = Enemy::new(); 
        enemy.create_enemy();
        Task::succeed(|| FFGame {
            player,
            enemy,

            ticks: 1,
            lkey: None,

            phealth: 100,
            ehealth: 60, 
            pwins: 0, 
            ewins: 0
        })
    }

    fn update(&mut self, _window: &Window) {

    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        if input.keys_pressed.len() != 0 {
            self.lkey = ai::input(input.keys_pressed[0]);
        } else if input.keys_released.len() == 2 { 
            self.lkey = None;
        }
    }

    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {    
        if timer.has_ticked() { 
            // check to see if they are touching each other
            let player_position = self.player.position[0];
            let enemy_position = self.enemy.position[0];
            let xdiff = player_position.0 - enemy_position.0;
            let ydiff = player_position.1 - enemy_position.1;
            if (xdiff <= 40.0) && (xdiff >= -40.0) &&
                (ydiff <= 40.0) && (ydiff >= -40.0) { 
                    Term::stdout().clear_last_lines(5).unwrap();
                    println!("\nPlayer health: {} Enemy health: {}", self.phealth, self.ehealth);
                    if self.player.is_moving() { 
                        self.ehealth -= 20; 
                    } else if self.enemy.is_moving() { 
                        self.phealth -= 20;
                    }
                    self.player.position.push(Position(20.0, 45.0));
                    self.enemy.position.push(Position(560.0, 90.0));
            } 

            self.ticks += 1; 
            self.player.move_to(self.lkey);
            self.player.position = Vec::from([*self.player.position.last().unwrap()]);    

            self.enemy.move_to(self.lkey);
            self.enemy.position = Vec::from([*self.enemy.position.last().unwrap()]);    

            frame.clear(Color{
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0
            });             
            self.enemy.draw_enemy(frame);
            self.player.draw_player(frame);

            if self.ticks >= 20 { 
                //println!("{}th tick: {:?}", self.ticks, self.player.position);            

                self.ticks = 0;                
            }

            if self.phealth <= 0 {  
                self.ewins += 1; 
                self.ehealth = 60; 
                self.phealth = 100;
                println!("\n\nPlayer lost ({}), Enemy won ({})\n", self.pwins, self.ewins);
            } else if self.ehealth <= 0 { 
                self.pwins += 1; 
                self.ehealth = 60; 
                self.phealth = 100;
                println!("\n\nEnemy lost ({}), Player won ({})\n", self.ewins, self.pwins);
            }
        }
    }
}

struct CustomInput {
    keys_pressed: Vec<KeyCode>,
    keys_released: Vec<KeyCode>
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            keys_pressed: vec![],
            keys_released: vec![]
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.push(key_code);
                    },
                    input::ButtonState::Released => {
                        self.keys_released.push(key_code);
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn clear(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }
}
