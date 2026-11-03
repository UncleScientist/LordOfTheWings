use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let data = include_str!("../input/everybody_codes_e2026_q06_p1.txt");
    let recipe = Recipe::new(data.lines());
    println!("Quest 6, part 1 = {}", recipe.count_ingredients_for("NULL"));
}

#[derive(Debug)]
struct Recipe {
    ingredients: HashMap<String, Vec<String>>,
}

impl Recipe {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut ingredients = HashMap::new();
        for line in lines {
            let (base, components) = line.split_once(':').unwrap();
            let components = components
                .split(',')
                .map(|c| c.to_string())
                .collect::<Vec<_>>();
            ingredients.insert(base.into(), components);
        }

        Self { ingredients }
    }

    fn count_ingredients_for(&self, start: &str) -> usize {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([start.to_string()]);

        while let Some(entry) = queue.pop_front() {
            if !seen.insert(entry.clone()) {
                continue;
            }
            if let Some(neighbors) = self.ingredients.get(&entry) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.clone());
                }
            }
        }

        seen.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = include_str!("../test-input/part1");
        let recipe = Recipe::new(data.lines());
        assert_eq!(7, recipe.count_ingredients_for("NULL"));
    }
}
