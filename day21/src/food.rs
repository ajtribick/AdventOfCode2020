use std::{error::Error, fmt, iter, rc::Rc};

use ahash::{AHashMap, AHashSet};

#[derive(Debug)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct FoodId(usize);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct AllergenId(usize);

#[derive(Debug)]
pub struct FoodProcessor {
    id_foods: AHashMap<FoodId, Rc<str>>,
    id_allergens: AHashMap<AllergenId, Rc<str>>,
    safe_counts: AHashMap<FoodId, usize>,
    allergen_possibilities: AHashMap<AllergenId, AHashSet<FoodId>>,
}

fn parse_foods(
    food_str: &str,
    food_ids: &mut AHashMap<Rc<str>, FoodId>,
    id_foods: &mut AHashMap<FoodId, Rc<str>>,
    safe_counts: &mut AHashMap<FoodId, usize>,
) -> AHashSet<FoodId> {
    let foods = food_str.split(' ').map(Rc::from);

    let mut line_foods = AHashSet::new();
    for food in foods {
        let mut food_id = FoodId(food_ids.len());
        food_ids
            .entry(Rc::clone(&food))
            .and_modify(|id| food_id = *id)
            .or_insert(food_id);
        id_foods.insert(food_id, food);
        line_foods.insert(food_id);
        safe_counts
            .entry(food_id)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    line_foods
}

fn parse_allergens(
    allergens_str: &str,
    allergen_ids: &mut AHashMap<Rc<str>, AllergenId>,
    id_allergens: &mut AHashMap<AllergenId, Rc<str>>,
) -> Vec<AllergenId> {
    let allergens = allergens_str
        .strip_suffix(')')
        .unwrap_or(allergens_str)
        .split(", ")
        .map(Rc::from);

    let mut line_allergens = Vec::new();
    for allergen in allergens {
        let mut allergen_id = AllergenId(allergen_ids.len());
        allergen_ids
            .entry(Rc::clone(&allergen))
            .and_modify(|id| allergen_id = *id)
            .or_insert(allergen_id);
        id_allergens.insert(allergen_id, allergen);
        line_allergens.push(allergen_id);
    }

    line_allergens
}

impl FoodProcessor {
    pub fn parse<S, I>(lines: I) -> Result<Self, ParseError>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let mut allergen_possibilities = AHashMap::new();
        let mut id_foods = AHashMap::new();
        let mut id_allergens = AHashMap::new();
        let mut safe_counts = AHashMap::new();

        let mut food_ids = AHashMap::new();
        let mut allergen_ids = AHashMap::new();

        for line_ref in lines {
            let line = line_ref.as_ref();
            let mut parts = line.splitn(2, " (contains ");

            let foods = parts.next().ok_or(ParseError("Missing foods list"))?;
            let line_foods = parse_foods(foods, &mut food_ids, &mut id_foods, &mut safe_counts);

            let allergens = parts.next().ok_or(ParseError("Missing allergens list"))?;
            let line_allergens = parse_allergens(allergens, &mut allergen_ids, &mut id_allergens);

            for allergen in line_allergens {
                allergen_possibilities
                    .entry(allergen)
                    .and_modify(|value: &mut AHashSet<FoodId>| {
                        value.retain(|s| line_foods.contains(s));
                    })
                    .or_insert_with(|| line_foods.clone());
            }
        }

        allergen_possibilities
            .values()
            .flat_map(|v| v.iter())
            .for_each(|v| {
                safe_counts.remove(v);
            });

        Ok(Self {
            id_foods,
            id_allergens,
            safe_counts,
            allergen_possibilities,
        })
    }

    pub fn safe_count(&self) -> usize {
        self.safe_counts.values().sum()
    }

    fn get_food_possibilities(&self) -> AHashMap<FoodId, AHashSet<AllergenId>> {
        let mut food_possibilities = AHashMap::with_capacity(self.id_foods.len());

        for (allergen_id, food_ids) in &self.allergen_possibilities {
            for &food_id in food_ids {
                food_possibilities
                    .entry(food_id)
                    .and_modify(|s: &mut AHashSet<AllergenId>| {
                        s.insert(*allergen_id);
                    })
                    .or_insert_with(|| iter::once(*allergen_id).collect());
            }
        }

        food_possibilities
    }

    fn build_food_map(
        &self,
        food_possibilities: &mut AHashMap<FoodId, AHashSet<AllergenId>>,
    ) -> Vec<(FoodId, Rc<str>)> {
        let mut food_map = Vec::with_capacity(self.id_foods.len());
        while !food_possibilities.is_empty() {
            let (food_id, allergen_id) = food_possibilities
                .iter()
                .find_map(|(food_id, allergen_ids)| {
                    if allergen_ids.len() == 1 {
                        Some((*food_id, *allergen_ids.iter().next().unwrap()))
                    } else {
                        None
                    }
                })
                .unwrap();

            food_map.push((food_id, self.id_allergens[&allergen_id].clone()));
            food_possibilities.remove(&food_id);
            food_possibilities.values_mut().for_each(|allergen_ids| {
                allergen_ids.remove(&allergen_id);
            });
        }

        food_map.sort_unstable_by(|(_, allergen_a), (_, allergen_b)| allergen_a.cmp(allergen_b));
        food_map
    }

    pub fn map_allergens(&self) -> String {
        let food_map = self.build_food_map(&mut self.get_food_possibilities());

        let mut foods = food_map
            .iter()
            .map(|(food_id, _)| self.id_foods[food_id].clone());

        let mut result = String::with_capacity(1024);
        if let Some(food) = foods.next() {
            result.push_str(&food);
            for food in foods {
                result.push(',');
                result.push_str(&food);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::FoodProcessor;

    const EXAMPLE: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn safe_count_test() {
        let processor = FoodProcessor::parse(EXAMPLE.lines()).unwrap();
        let result = processor.safe_count();
        assert_eq!(result, 5);
    }

    #[test]
    fn map_allergens_test() {
        let processor = FoodProcessor::parse(EXAMPLE.lines()).unwrap();
        let result = processor.map_allergens();
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl");
    }
}
