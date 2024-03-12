extern crate coffee;
use coffee::graphics::{
    Color, Frame, Window, WindowSettings
};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};
use coffee::load::Task;
use coffee::{Game, Timer};
mod terrain;
use terrain::generation;
mod ai; 
use ai::{Player, Enemy};

fn main() {
    FFGame::run(WindowSettings {
        title: String::from("player"),
        size: (900, 800),
        resizable: true,
        maximized: false,
        fullscreen: false,
    }).unwrap();
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(f32, f32);






struct FFGame {
    player: Player,
    score: u16,
    xp: u32,
    ticks: u64,
    health: u8,
    lkey: Option<KeyCode>,
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
        Task::succeed(|| FFGame {
           player,
           score: 0, 
           xp: 0,
           ticks: 1,
           health: 100, 
           lkey: None,
        })
    }

    fn update(&mut self, _window: &Window) {

    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        if input.keys_pressed.len() != 0 {
            let key = input.keys_pressed[0];
            match key {
                KeyCode::W => {
                    self.lkey = Some(key);
                }
                KeyCode::A => {
                    self.lkey = Some(key);
                }
                KeyCode::S => {
                    self.lkey = Some(key);
                }
                KeyCode::D => {
                    self.lkey = Some(key);
                }
                _ => { 
                    self.lkey = None; 
                },
            }
        } else if input.keys_released.len() == 1 { 
            self.lkey = None;
        }
    }
    fn draw(&mut self, frame: &mut Frame, timer: &Timer) {
        if timer.has_ticked() && !self.player.died() {    
            self.ticks += 1; 
            self.player.move_to(self.lkey);
            self.player.position = Vec::from([*self.player.position.last().unwrap()]);        

            frame.clear(Color{
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0
            });              
            self.player.draw_player(frame);

            if self.ticks >= 20 { 
                println!("{}th tick: {:?}", self.ticks, self.player.position);
                self.ticks = 0;                
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