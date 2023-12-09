#![warn(clippy::all)]

struct Set{
    pub red: u32,
    pub green: u32,
    pub blue: u32 
}

struct Game{
    pub id: usize,
    pub sets: Vec<Set>
}
impl Set{
    pub fn is_possible(&self, game:&Set)->bool{
        self.red <= game.red &&
        self.green <= game.green &&
        self.blue <= game.blue
    }
    pub fn get_power(&self) -> u32{
        self.red * self.green * self.blue
    }
}
impl Game {
    pub fn is_possible(&self, reference:&Set) -> bool{
        self.sets.iter().all(|s|s.is_possible(reference))
    }
    pub fn min_set(&self) -> Set{
        Set { 
            red: self.sets.iter().max_by_key(|s|s.red).unwrap().red, 
            green: self.sets.iter().max_by_key(|s|s.green).unwrap().green, 
            blue: self.sets.iter().max_by_key(|s|s.blue).unwrap().blue 
        }
    }
}

fn parse_games(input: &str)->Vec<Game>{
    input.lines().map(|l|->Game{
        let mut s = l.split(": ");
        Game{
            id: s.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap(),
            sets: s.next().unwrap().split("; ").map(|set|{
                let (mut red, mut green, mut blue): (u32,u32,u32) = (0,0,0);
                for draw in set.split(", "){
                    let mut sd = draw.split(" ");
                    let n = sd.next().unwrap().parse::<u32>().unwrap();
                    let t = sd.next().unwrap();
                    match t {
                        "red" => {red = n},
                        "green" => {green = n},
                        _ => {blue = n}
                    }
                }
                Set{
                    red,green,blue
                }
            }).collect()
        }
    }).collect()

}

fn main() {
    let input = include_str!("resources/input");
    let reference = Set{
        red: 12,
        green: 13,
        blue: 14, 
    };
    let games: Vec<Game> = parse_games(input);
    println!("{}",games.iter().filter(|g|g.is_possible(&reference)).map(|g|g.id).sum::<usize>());
    println!("{}", games.iter().map(|g|g.min_set().get_power()).sum::<u32>());

}