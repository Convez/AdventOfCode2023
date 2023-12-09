#![warn(clippy::all)]
use itertools::Itertools;

fn main() {
    let input = include_str!("resources/input");
    let mut cards = input.lines().enumerate().map(|l|(l.0,l.1.split(':').last().unwrap()))
    .map(|l|{
        let mut ns = l.1.split("|");
        let winnings = ns.next().unwrap().split_whitespace().map(|n|n.parse::<u32>().unwrap()).collect_vec();
        let having = ns.last().unwrap().split_whitespace().map(|n|n.parse::<u32>().unwrap()).collect_vec();
        (l.0+1,winnings,having,1)
    }).collect_vec();
    println!("{:?}",cards);
    let result = cards.iter()
    .map(|c|{
        c.2.iter().filter(|n| c.1.contains(n))
        .collect_vec()
    }).filter(|n|n.len()>0).collect_vec();
    println!("{:?}",result);
    let rs = result.iter().map(|n|u32::pow(2,n.len() as u32-1)).collect_vec();
    println!("{:?}",rs);
    println!("{}",rs.iter().sum::<u32>());
    for ix in 0..cards.len(){
        let c = cards[ix].clone();
        let n_won = c.2.iter().filter(|n| c.1.contains(n)).collect_vec().len();
        for i in c.0..(c.0+n_won){
            cards[i].3+=c.3;
        }
    }
    println!("{:?}",cards);
    println!("{:?}",cards.iter().map(|c|c.3).sum::<usize>());
}