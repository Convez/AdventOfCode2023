#![warn(clippy::all)]
fn parse1(input: &str) -> u32{
    let mut s = 0;
    for line in input.lines(){
        let mut first:u32=0;
        let mut last:u32=0;
        for c in line.chars(){
            if c.is_numeric() {
                first = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev(){
            if c.is_numeric() {
                last = c.to_digit(10).unwrap();
                break;
            }
        }
        s += first*10+last;
    }
    s
}

fn parse2(input: &str) -> u32{
    let nums = [
    ("one","1"),
    ("two","2"), 
    ("three","3"), 
    ("four","4"), 
    ("five","5"), 
    ("six","6"), 
    ("seven","7"), 
    ("eight","8"), 
    ("nine","9")];
    let input2 = input.lines().map(|line|{
        let mut line3 = line.to_string();
        for n in nums{
            line3 = line3.replace(n.0, (n.0.to_string()+n.1+n.0).as_str());
        }
        line3+"\n"
    }).collect::<String>();
    // println!("{}",input2);
    parse1(input2.as_str())
}

fn main() {
    let input = include_str!("resources/input");
    println!("{}",parse1(input));
    println!("{}",parse2(input));
}