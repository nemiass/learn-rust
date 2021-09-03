#![allow(dead_code)] // para evitar los warnings de variables no usadas o no leidas

const WIDHT: u32 = 500;
const HEIGHT: u32 = 500;
const ROWS: u32 = 9;
const COLS: u32 = 9;

struct Ant {
    x: i32,
    y: i32,
    direction: i8,
}

impl Ant {
    fn mod_array(& self, table: &mut [[i8; ROWS as usize]; COLS as usize]) {
        table[0][0] = 1;
    }
}
// fn start(& self, table: [[i8; ROWS as usize]; COLS as usize]) {

// }
struct Game {
    table: [[i8; ROWS as usize]; COLS as usize],
    ant: Ant,
}

impl Game {
    fn test(&mut self) {
        self.ant.mod_array(&mut self.table);
    }
}

fn main() {
    let mut table = [[0; ROWS as usize]; COLS as usize];

    let mut ant = Ant {
        x: (ROWS / 2) as i32,
        y: (COLS / 2) as i32,
        direction: 0,
    };


    let mut game = Game {
        table: table,
        ant: ant,
    };

    game.test();

    println!("{:?}", game.table);
}
