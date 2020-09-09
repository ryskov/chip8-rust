#[derive(Debug, Default)]
pub struct Display {
    pub image: Box<Vec<Vec<u8>>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            image: Box::new(vec![vec![0; 64]; 32]),
        }
    }

    pub fn draw(&mut self, x: u8, y: u8, bytes: Vec<u8>, v_flag: &mut bool) {
        for byte_pos in 0..bytes.len() {
            let y = (y as usize + byte_pos) % 32;
            for bit_pos in 0..8 {
                let x = (x as usize + bit_pos) % 64;
                let draw = (bytes[byte_pos] >> (7 - bit_pos)) & 1;

                if draw == 1 && self.image[y][x] == 0b1 {
                    *v_flag = true;
                }
                self.image[y][x] ^= draw;
            }
        }

        self.print();
    }

    fn print(&self) {
        print!("{}[2J", 27 as char);
        for row in self.image.iter() {
            for column in row.iter() {
                if *column == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
