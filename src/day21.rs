extern crate multimap;
extern crate peg;

use itertools::__std_iter::Filter;
use multimap::MultiMap;
use std::collections::HashMap;
use std::fmt;
use std::slice::Iter;

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

    // include_str!("../day21_test.txt")
    include_str!("../day21.txt")
        .split("\n")
        .map(|l| food_list_parser::line(l).ok().unwrap())
        .for_each(|l: Line| {
            l.food.iter().for_each(|food| {
                l.allergen.iter().for_each(|allergen| {
                    if allergen_per_ingredient.contains_key(allergen) {
                        let option = allergen_per_ingredient.get_mut(allergen).unwrap();

                        *option.entry(food.clone()).or_insert(0) += 1;
                    } else {
                        allergen_per_ingredient.insert(allergen.clone(), HashMap::new());
                    }

                    if ingredient_per_allergen.contains_key(food) {
                        let option = ingredient_per_allergen.get_mut(food).unwrap();
                        *option.entry(allergen.clone()).or_insert(0) += 1;
                    } else {
                        ingredient_per_allergen.insert(food.clone(), HashMap::new());
                    }
                })
            });
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
