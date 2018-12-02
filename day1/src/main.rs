use std::io::Lines;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    println!("{}", sum(file.lines()));
}

fn sum(i: Lines<std::io::BufReader<&std::fs::File>>) -> i32{
    let mut total = 0;

    for s in i{
        match s.unwrap().parse::<i32>() {
            Ok(delta) => total += delta,
            Err(_e) => (),//println!("didn't like {} because {}", t, e),
        }
    }
    return total
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test1(){
//         let args = "a".to_string();
//         assert_eq!(sum(args), 3);
//     }

// }
