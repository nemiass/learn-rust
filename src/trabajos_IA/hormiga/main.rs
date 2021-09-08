#![allow(dead_code)] // para evitar los warnings de variables no usadas o no leidas

const WIDHT: usize = 500;
const HEIGHT: usize = 500;
const ROWS: usize = 9;
const COLS: usize = 9;

struct Ant {
    x: i32,
    y: i32,
    direction: i32,
}

impl Ant {
    fn mod_array(&self, table: &mut [[i8; ROWS as usize]; COLS as usize]) {
        table[0][0] = 1;
    }
}
// fn start(& self, table: [[i8; ROWS as usize]; COLS as usize]) {

// }
struct Game {
    table: [[i8; ROWS]; COLS],
    ant: Ant,
}

impl Game {
    fn test(&mut self) {
        self.ant.mod_array(&mut self.table);
    }

    fn render(&self) {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.table[row][col] == 1 {
                    print!("*");
                } else {
                    print!("*")
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut table = [[0; ROWS]; COLS];

    let mut ant = Ant {
        x: (ROWS / 2) as i32,
        y: (COLS / 2) as i32,
        direction: 0,
    };

    let mut game = Game {
        table: table,
        ant: ant,
    };

    loop {
        game.render();
    }
}
