use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q06_p1.txt").expect("file");
    let recipe = Recipe::new(data.lines());
    println!("Quest 6, part 1 = {}", recipe.count_ingredients_for("NULL"));

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q06_p2.txt").expect("file");
    let recipe = Recipe::new(data.lines());
    println!("Quest 6, part 2 = {}", recipe.count_used_by("NULL"));

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q06_p3.txt").expect("file");
    let recipe = Recipe::new(data.lines());
    println!("Quest 6, part 3 = {}", recipe.uses_two_nulls());
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

    fn count_used_by(&self, start: &str) -> usize {
        self.used_by(start).len()
    }

    fn used_by(&self, start: &str) -> HashSet<String> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::from(
            self.ingredients
                .iter()
                .filter_map(|(k, v)| {
                    if v.contains(&start.to_string()) {
                        Some(k)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect::<Vec<_>>(),
        );

        while let Some(entry) = queue.pop_front() {
            if !seen.insert(entry.clone()) {
                continue;
            }

            for source in self.ingredients.iter().filter_map(|(k, v)| {
                if v.contains(&entry.to_string()) {
                    Some(k)
                } else {
                    None
                }
            }) {
                if !seen.contains(source) && source != start {
                    queue.push_back(source.clone());
                }
            }
        }

        seen
    }

    fn uses_two_nulls(&self) -> usize {
        let v1 = self.used_by("NULLv1");
        let v2 = self.used_by("NULLv2");
        let v3 = self.used_by("NULLv3");

        let v1v2 = v1.intersection(&v2);
        let v1v3 = v1.intersection(&v3);
        let v2v3 = v2.intersection(&v3);

        v1v2.chain(v1v3).chain(v2v3).collect::<HashSet<_>>().len()
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

    #[test]
    fn test_part2() {
        let data = include_str!("../test-input/part2");
        let recipe = Recipe::new(data.lines());
        assert_eq!(6, recipe.count_used_by("NULL"));
    }

    #[test]
    fn test_part3() {
        let data = include_str!("../test-input/part3");
        let recipe = Recipe::new(data.lines());
        assert_eq!(3, recipe.uses_two_nulls());
    }
}
