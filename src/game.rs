use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};

use draw::{draw_block, draw_rectangle};
use snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const PLAYER_1_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const PLAYER_2_COLOR: Color = [0.00, 0.50, 0.80, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

#[derive(Debug)]
enum Players {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug)]
struct PlayerDirection {
    player: Players,
    direction: Direction,
}

impl PlayerDirection {
    pub fn new(player: Players, direction: Direction) -> PlayerDirection {
        PlayerDirection {
            player: player,
            direction: direction,
        }
    }
}

pub struct Game {
    player_1: Snake,
    player_2: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    score: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            player_1: Snake::new(1, 1, PLAYER_1_COLOR),
            player_2: Snake::new(3, 3, PLAYER_2_COLOR),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            score: 0,
        }
    }

    fn get_player_direction(&self, key: piston_window::Key) -> Option<PlayerDirection> {
        let player = match key {
            Key::Up => Some(PlayerDirection::new(Players::PlayerOne, Direction::Up)),
            Key::Down => Some(PlayerDirection::new(Players::PlayerOne, Direction::Down)),
            Key::Left => Some(PlayerDirection::new(Players::PlayerOne, Direction::Left)),
            Key::Right => Some(PlayerDirection::new(Players::PlayerOne, Direction::Right)),
            Key::W => Some(PlayerDirection::new(Players::PlayerTwo, Direction::Up)),
            Key::S => Some(PlayerDirection::new(Players::PlayerTwo, Direction::Down)),
            Key::A => Some(PlayerDirection::new(Players::PlayerTwo, Direction::Left)),
            Key::D => Some(PlayerDirection::new(Players::PlayerTwo, Direction::Right)),
            _ => None,
        };
        player
    }

    fn update_snake(&mut self, player_dir: PlayerDirection) {
        match player_dir.player {
            Players::PlayerOne => {
                if player_dir.direction != self.player_1.head_direction().opposite() {
                    if self.check_if_snake_alive_1(Some(player_dir.direction)) {
                        self.check_overflow_snake_1();
                        self.player_1.move_forward(Some(player_dir.direction));
                        self.check_eating_1();
                    } else {
                        self.game_over = true;
                    }
                    self.waiting_time = 0.0;
                }
            }
            Players::PlayerTwo => {
                if player_dir.direction != self.player_2.head_direction().opposite() {
                    if self.check_if_snake_alive_2(Some(player_dir.direction)) {
                        self.check_overflow_snake_2();
                        self.player_2.move_forward(Some(player_dir.direction));
                        self.check_eating_2();
                    } else {
                        self.game_over = true;
                    }
                    self.waiting_time = 0.0;
                }
            }
        }
    }

    pub fn key_pressed(&mut self, key: piston_window::Key) {
        if self.game_over {
            return;
        }

        let player_direction = self.get_player_direction(key);

        match player_direction {
            Some(dir) => self.update_snake(dir),
            None => println!("None"),
        }

        return;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.player_1.draw(con, g);
        self.player_2.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(PlayerDirection::new(
                Players::PlayerOne,
                self.player_1.head_direction(),
            ));
            self.update_snake(PlayerDirection::new(
                Players::PlayerTwo,
                self.player_2.head_direction(),
            ));
        }
    }

    fn check_overflow_snake_1(&mut self) -> bool {
        let (head_x, head_y): (i32, i32) = self.player_1.head_position();

        if head_x < 0 {
            self.player_1.overflow_switch(self.width, head_y);
        } else if head_y < 0 {
            self.player_1.overflow_switch(head_x, self.height);
        } else if head_x > self.width {
            self.player_1.overflow_switch(0, head_y);
        } else if head_y > self.height {
            self.player_1.overflow_switch(head_x, 0);
        }
        return true;
    }

    fn check_overflow_snake_2(&mut self) -> bool {
        let (head_x, head_y): (i32, i32) = self.player_2.head_position();

        if head_x < 0 {
            self.player_2.overflow_switch(self.width, head_y);
        } else if head_y < 0 {
            self.player_2.overflow_switch(head_x, self.height);
        } else if head_x > self.width {
            self.player_2.overflow_switch(0, head_y);
        } else if head_y > self.height {
            self.player_2.overflow_switch(head_x, 0);
        }
        return true;
    }

    fn check_eating_1(&mut self) {
        let (head_x, head_y): (i32, i32) = self.player_1.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.score = self.score + 1;
            self.food_exists = false;
            self.player_1.restore_tail();
        }
    }

    fn check_eating_2(&mut self) {
        let (head_x, head_y): (i32, i32) = self.player_2.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.score = self.score + 1;
            self.food_exists = false;
            self.player_2.restore_tail();
        }
    }

    fn check_if_snake_alive_1(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.player_1.next_head(dir);
        if self.player_1.overlap_tail(next_x, next_y) {
            return false;
        }
        return true;
    }

    fn check_if_snake_alive_2(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.player_2.next_head(dir);
        if self.player_2.overlap_tail(next_x, next_y) {
            return false;
        }
        return true;
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.player_1.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn restart(&mut self) {
        self.player_1 = Snake::new(2, 2, PLAYER_1_COLOR);
        self.player_2 = Snake::new(3, 3, PLAYER_2_COLOR);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
