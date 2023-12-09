#![warn(clippy::all)]
use itertools::Itertools;
use range_ext::intersect::Intersect;

fn main() {
    let input = include_str!("resources/input");
    let mut lines = input.lines().collect_vec();
    let binding = std::iter::repeat(".").take(lines.first().unwrap().len()).collect::<String>();
    lines.insert(0, binding.as_str());
    lines.push( binding.as_str());
    let lines2 = lines.iter().map(|l|{
        let mut b = l.to_string();
        b.insert(0, '.');
        b.push('.');
        b
    }).collect_vec();
    let mut num_loc= Vec::<(usize,usize,usize,String)>::new();
    for (ln, line) in lines2.iter().enumerate(){
        let v = line.chars().enumerate()
        .filter(|c|c.1.is_numeric()).map(|c|(c.0,c.0,c.1.to_string()))
        .reduce(|acc,e|{
            if e.0 == acc.0+1{
                return (e.0, acc.1,acc.2+e.2.as_str())
            }else{
                num_loc.push((ln,acc.1,acc.0,acc.2));
                return e;
            }
        });
        if v.is_some(){
            let v1 = v.unwrap();
            num_loc.push((ln,v1.1,v1.0,v1.2));
        }
    }
    // println!("{:?}",num_loc);
    let mut acc = 0;
    for n in num_loc.iter(){
        let line_n = n.0;
        let ns = n.1;
        let nf = n.2;
        for i in (line_n-1)..=(line_n+1){
            for j in (ns-1)..=(nf+1){
                let c = lines2[i].chars().nth(j).unwrap();
                if !c.is_numeric() && c!='.'{
                    acc += n.3.parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("{acc}");
    let gears = lines2.iter().enumerate()
    .flat_map(|(row, l)|l.chars().enumerate().map(move |(col,c)|(row,col,c)))
    .filter(|(_,_,c)|c.eq(&'*')).collect_vec();
    // println!("{:?}",gears);
    let mut acc2 = 0;
    for g in gears{
        let eligible = num_loc.iter()
        .filter(|(row,start,finish,_)|((g.0-1)..=(g.0+1)).contains(row) && ((g.1-1)..=(g.1+1)).does_intersect(&(start.clone()..=finish.clone())))
        .collect_vec();
        if eligible.len() == 2{
            acc2 += eligible[0].3.parse::<u32>().unwrap() * eligible[1].3.parse::<u32>().unwrap();
        }
    }
    println!("{acc2}");
}