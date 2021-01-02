use itertools::Itertools;

pub fn day_nine() {
    // let number = include_str!("../day9.txt")
    //     .split("\n")
    //     .filter_map(|w| w.parse::<i32>().ok())
    //     .collect::<Vec<i32>>()
    //     .windows(26)
    //     // .inspect(|r| println!("{:?}", r))
    //     .find(|x| {
    //         // println!("{} ", x[0..25].len());
    //
    //         !x[0..25]
    //             .iter()
    //             .combinations(2)
    //             .any(|y| y.iter().map(|z| **z).sum::<i32>() == x[25])
    //     })
    //     .map(|x| x[25])
    //     .unwrap();

    let vec = include_str!("../day9.txt")
        .split("\n")
        .filter_map(|w| w.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    let window = (2..41682219 as usize).find(|w| {
        vec.windows(*w)
            // .inspect(|r| println!("{:?}", r))
            .find(|x| x.iter().sum::<i64>() == 41682220)
            .is_some()
    });

    let range = vec
        .windows(window.unwrap())
        // .inspect(|r| println!("{:?}", r))
        .find(|x| x.iter().sum::<i64>() == 41682220)
        .unwrap();

    dbg!(range.to_vec().iter().max().unwrap() + range.to_vec().iter().min().unwrap());
    // 41682220

    // .windows(25)
    // .inspect(|x| println!("{}", x))
    //
    // .find(|n| {
    //     !preamble
    //         .iter()
    //         .combinations(2)
    //         .any(|x| x.iter().map(|x| **x).sum::<i32>() == *n)
    // })
    // .unwrap();

    // dbg!(w);
}
