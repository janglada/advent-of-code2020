use std::cmp::Ordering;
use std::cmp::Ordering::Less;

fn read(txt: &str) -> (usize, Vec<usize>) {
    let mut chars = txt.split("\n");
    let timesmap = chars
        .next()
        .unwrap()
        .chars()
        .as_str()
        .parse::<usize>()
        .ok()
        .unwrap();

    let mut buses: Vec<usize> = chars
        .next()
        .map(|x| x.chars().as_str())
        .unwrap()
        .to_string()
        .split(",")
        .filter(|x| !x.contains('x'))
        .map(|x| x.parse::<usize>().ok().unwrap())
        .collect();

    // println!("{}", timesmap);
    println!("{:?}", buses);
    (timesmap, buses)
}
pub fn day_thirteen() -> Result<(), Box<std::error::Error>> {
    let (timestamp, buses) = read(include_str!("../day13.txt"));

    let a: Vec<usize> = buses
        .iter()
        .map(|id| ((timestamp / id) + 1) * id - timestamp)
        .collect();

    let index_of_min: Option<usize> = a
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index);

    println!("{:?}", a);
    // println!("{:?}", b);
    println!("{:?}", index_of_min.unwrap());
    println!("{:?}", buses.get(index_of_min.unwrap()).unwrap());

    let bus = buses.get(index_of_min.unwrap()).unwrap();
    let times = a.get(index_of_min.unwrap()).unwrap();

    println!("{}", bus * times);
    Ok(())
}
