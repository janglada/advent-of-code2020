extern crate peg;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

peg::parser! {
  grammar food_list_parser() for str
  {
    pub rule word() -> String
      = i:$(['a'..='z']+) { i.to_owned() }

    pub rule ingredient_list() -> Vec<String>
      = d:word() ** " " {
            d
      }

    pub rule line() -> Line
      = l:ingredient_list()  [_] "(" a:allergen_container() ")" {
            Line {
                food:l,
                allergen:a
            }
      }
    pub rule allergen_container() -> Vec<String>
      = "contains" " " a:allergen()  {
            a
      }

    pub rule allergen() -> Vec<String>
      = d:word() ** ", " {
            d
      }
    }

}

pub struct Line {
    food: Vec<String>,
    allergen: Vec<String>,
}
#[derive(Debug)]
struct Item<T: AsRef<str>> {
    num: u32,
    word: T,
}

impl<T> Item<T>
where
    T: AsRef<str>,
{
    pub fn add(&mut self) {
        self.num = self.num + 1;
    }
}

// impl fmt::Display for Item<T: AsRef<str>> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({} : {})", self.word, self.num)
//     }
// }

pub fn run() {
    let mut allergen_per_ingredient: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut ingredient_per_allergen: HashMap<String, HashMap<String, u32>> = HashMap::new();

    let mut allergen_count: HashMap<String, u32> = HashMap::new();

    let recipes: Vec<Line> = include_str!("../day21.txt")
        // include_str!("../day21.txt")
        .split("\n")
        .map(|l| food_list_parser::line(l).ok().unwrap())
        .collect();

    recipes.iter().for_each(|l| {
        l.food.iter().for_each(|food| {
            l.allergen.iter().for_each(|allergen| {
                if allergen_per_ingredient.contains_key(allergen) {
                    let option = allergen_per_ingredient.get_mut(allergen).unwrap();

                    *option.entry(food.clone()).or_insert(0) += 1;
                } else {
                    let mut map = HashMap::new();
                    map.insert(food.clone(), 1);
                    allergen_per_ingredient.insert(allergen.clone(), map);
                }

                if ingredient_per_allergen.contains_key(food) {
                    let option = ingredient_per_allergen.get_mut(food).unwrap();
                    *option.entry(allergen.clone()).or_insert(0) += 1;
                } else {
                    let mut map = HashMap::new();
                    map.insert(allergen.clone(), 1);
                    ingredient_per_allergen.insert(food.clone(), map);
                }
            })
        });
        l.allergen.iter().for_each(|allergen| {
            *allergen_count.entry(allergen.clone()).or_insert(0) += 1;
        })
    });

    ingredient_per_allergen
        .iter()
        // .filter(|(k, v)| v.len() == 1)
        .for_each(|(k, v)| println!("({} : {:?})", k, v));
    println!("---------------------------");
    allergen_per_ingredient
        .iter()
        // .filter(|(k, v)| v.len() == 1)
        .for_each(|(k, v)| println!("({} : {:?})", k, v));
    println!("---------------------------");
    println!("{:?} ", allergen_count);
    // let ingredients: Vec<String> = ingredient_per_allergen
    //     .iter()
    //     .filter_map(|(k, v)| {
    //         if v.values().into_iter().all(|n| *n == 1) {
    //             Some(k.clone())
    //         } else {
    //             None
    //         }
    //     })
    //     //  .map(|(&k, &v)| k)
    //     .collect();

    let mut set: HashSet<String> = HashSet::new();
    ingredient_per_allergen.keys().for_each(|k| {
        set.insert(k.clone());
    });

    let mut removed: HashSet<&String> = HashSet::new();
    //let mut s = 1;
    // while removed.len() != s {
    //     s = removed.len();
    //     println!("xxxx");
    //     allergen_per_ingredient
    //         .iter()
    //         .for_each(|(allergen, foods)| {
    //             let max = foods
    //                 .iter()
    //                 .filter(|(f, _)| !removed.contains(f))
    //                 .sorted_by(|(k1, v1), (k2, v2)| Ord::cmp(&v2, &v1))
    //                 .next()
    //                 .unwrap();
    //
    //             let x: Vec<(&String, &u32)> = foods.iter().filter(|(k, &v)| v == *max.1).collect();
    //             if x.len() == 1 {
    //                 set.remove(x.get(0).unwrap().0);
    //                 removed.insert(x.get(0).unwrap().0);
    //             }
    //         });
    // }

    // let mut deleted = allergen_per_ingredient
    //     .iter_mut()
    //     .for_each(|(allergen, foods)| {
    //         foods.iter_mut().for_each(|(food, count)| {
    //             match ingredient_per_allergen.get(food).unwrap().get(allergen) {
    //                 None => {
    //                     println!("{} does not contain allergen(I) {}", food, allergen);
    //                 }
    //                 Some(c) => {
    //                     if c > count {
    //                         set.remove(food);
    //                         println!("{} does  contain allergen(II) {}", food, allergen);
    //                     }
    //                 }
    //             }
    //         })
    //     });

    ingredient_per_allergen
        .iter()
        .filter(|(k, a)| {
            a.iter()
                .any(|(a1, v1)| v1 == allergen_count.get(a1).unwrap())
        })
        .for_each(|(k, a)| {
            set.remove(k);
        });

    println!("---------------------------");
    println!("REMAINING FOODS {:?}", set);

    let num: usize = recipes
        .iter()
        .map(|l| l.food.iter().filter(|&f| set.contains(f)).count())
        .sum();
    // let num: u32 = set
    //     .iter()
    //     .map(|f| {
    //         ingredient_per_allergen
    //             .get(f)
    //             .unwrap()
    //             .values()
    //             .sum::<u32>()
    //     })
    //     .sum();

    println!("{}", num);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_food() {
        let ingredients = food_list_parser::ingredient_list("mxmxvkd kfcds sqjhc nhms");

        assert_eq!(true, ingredients.is_ok());
        let opt = ingredients.ok();
        assert_eq!(true, opt.is_some());
        assert_eq!("mxmxvkd", opt.unwrap().get(0).unwrap());
    }

    #[test]
    fn test_allergen() {
        let allergens = food_list_parser::allergen("dairy, fish");

        assert_eq!(true, allergens.is_ok());
        let opt = allergens.ok();
        assert_eq!(true, opt.is_some());
        assert_eq!("dairy", opt.unwrap().get(0).unwrap());
    }

    #[test]
    fn test_line() {
        let line = food_list_parser::line("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");

        // let r = match line {
        //     Ok(r) => {}
        //     Err(e) => {
        //         println!("{}", e)
        //     }
        // };
        assert_eq!(true, line.is_ok());
        let opt = line.ok();
        assert_eq!(true, opt.is_some());
        let l: Line = opt.unwrap();
        println!("{:?} {:?}", l.food, l.allergen)
    }
}
