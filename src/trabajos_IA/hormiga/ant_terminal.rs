#![allow(dead_code)] // para evitar los warnings de variables no usadas o no leidas

const ROWS: usize = 40;
const COLS: usize = 40;

const OESTE: usize = 0;
const SUR: usize = 1;
const ESTE: usize = 2;
const NORTE: usize = 3;

struct Ant {
    x: i32,
    y: i32,
    direction: usize,
}

impl Ant {

    fn avanzar(&mut self, table: &mut [[i32; ROWS]; COLS]) {
        if table[self.x as usize][self.y as usize] == 0 {
            table[self.x as usize][self.y as usize] = 1;
            self.move_left();
        } else {
            table[self.x as usize][self.y as usize] = 0;
            self.move_right();
        }
    }

    fn move_left(&mut self) {
        match self.direction {
            OESTE => {
                self.direction = SUR;
                self.x = (self.x + 1) % ROWS as i32;
            }
            SUR => {
                self.direction = ESTE;
                self.y = (self.y + 1) % COLS as i32;
            }
            ESTE => {
                self.direction = NORTE;
                self.x = (self.x - 1 + ROWS as i32) % ROWS as i32;
            }
            NORTE => {
                self.direction = OESTE;
                self.y = (self.y - 1 + COLS as i32) % COLS as i32;
            }
            _ => {}
        }
    }

    fn move_right(&mut self) {
        match self.direction {
            OESTE => {
                self.direction = NORTE;
                self.x = (self.x - 1 + ROWS as i32) % ROWS as i32
            }
            SUR => {
                self.direction = OESTE;
                self.y = (self.y - 1 + COLS as i32) % COLS as i32;
            }
            ESTE => {
                self.direction = SUR;
                self.x = (self.x + 1) % ROWS as i32;
            }
            NORTE => {
                self.direction = ESTE;
                self.y = (self.y + 1) % COLS as i32;
            }
            _ => {}
        }
    }
}

struct Game {
    table: [[i32; ROWS]; COLS],
    ant: Ant,
}

impl Game {

    fn render(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.table[row][col] == 1 {
                    print!("  *");
                } else {
                    print!("   ");
                }
            }
            println!();
        }
    }

    fn update(&mut self) {
        self.ant.avanzar(&mut self.table);
    }
}

fn main() {
    let table = [[0; ROWS]; COLS];

    let ant = Ant {
        x: (ROWS / 2) as i32,
        y: (COLS / 2) as i32,
        direction: 0,
    };

    let mut game = Game {
        table: table,
        ant: ant,
    };

    loop {
        game.update();
        game.render();
        std::thread::sleep(std::time::Duration::from_millis(2));
        //print!("\x1B[2J\x1B[1;1H"); // desplaza en la terminal, pero no lo borra
        print!("{esc}c", esc = 27 as char); // limpiando terminal a la antiguad
    }
}
