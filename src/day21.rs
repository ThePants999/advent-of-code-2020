use std::collections::{HashSet, HashMap};

pub fn day21(input_lines: &[String]) -> (u64, u64) {
    let mut allergens: HashSet<String> = HashSet::new();
    let mut ingredients: HashMap<String, Ingredient> = HashMap::new();
    let mut foods: Vec<Food> = input_lines.iter().map(|line| Food::parse(line)).collect();
    for (index, food) in foods.iter().enumerate() {
        allergens.extend(food.allergens.iter().cloned());
        for ingredient_name in &food.ingredients {
            let ingredient = ingredients.entry(ingredient_name.clone()).or_insert_with(|| Ingredient::new(ingredient_name.clone()));
            ingredient.appears_in.insert(index);
        }
    }
    // To begin with, assume any ingredient could contain any allergen.
    for ingredient in ingredients.values_mut() {
        ingredient.candidate_allergens.extend(allergens.iter().cloned());
    }
    // For each ingredient, strike out any allergens contained in foods that don't contain
    // this ingredient.
    for ingredient in ingredients.values_mut() {
        let foods_not_containing_ingredient: Vec<usize> = (0..foods.len()).filter(|index| !ingredient.appears_in.contains(index)).collect();
        for index in foods_not_containing_ingredient {
            ingredient.candidate_allergens.retain(|allergen| !foods[index].allergens.contains(allergen));
        }
    }
    // Now, identify safe ingredients - get the part 1 answer, and then forget about them.
    let ingredients_with_no_allergens: Vec<String> = ingredients.values().filter_map(|ingredient| {
        if ingredient.candidate_allergens.is_empty() {
            Some(ingredient.name.clone())
        } else {
            None
        }
    }).collect();
    let mut part1 = 0u64;
    for ingredient in ingredients_with_no_allergens {
        for food in foods.iter_mut() {
            part1 += 1;
            food.ingredients.remove(&ingredient);
            ingredients.remove(&ingredient);
        }
    }

    // We now have only unsafe ingredients. Start by deriving two pieces of information:
    // -  If any food contains as many allergens as unsafe ingredients, each of those
    //    ingredients MUST contain one of those allergens - so those ingredients cannot
    //    contain any other allergens.
    //    1, 2 (contains A, B)
    //    2, 3 (contains C)
    //    Because of the first food, we can eliminate 2 from the second food.
    //    (Logically this also tells us that those allergens cannot be contained in any
    //    other ingredients, but we don't bother tracking that in this implementation.)
    // -  If, after excluding such impossibilities, a food contains only one allergen
    //    and one candidate ingredient, it's a match. So, in the example above, we can
    //    match 3 = C.
    let mut matched_ingredients: HashSet<String> = HashSet::with_capacity(allergens.len());
    let mut matches: HashMap<String, String> = HashMap::with_capacity(allergens.len());
    while matches.len() < allergens.len() {
        for food in foods.iter_mut() {
            food.ingredients.retain(|ingredient_name| !matched_ingredients.contains(ingredient_name));
            food.allergens.retain(|allergen_name| !matches.contains_key(allergen_name));
            if food.ingredients.len() == food.allergens.len() {
                for ingredient in ingredients.values_mut() {
                    if !food.ingredients.contains(&ingredient.name) {
                        // This ingredient is no longer a candidate for any of this food's allergens.
                        ingredient.candidate_allergens.retain(|allergen_name| !food.allergens.contains(allergen_name));
                    } else {
                        // This ingredient is no longer a candidate for any allergens other than those
                        // in this food.
                        ingredient.candidate_allergens.retain(|allergen_name| food.allergens.contains(allergen_name));
                    }
                }
            }
        }
        for ingredient in ingredients.values_mut() {
            ingredient.candidate_allergens.retain(|allergen_name| !matches.contains_key(allergen_name));
            if ingredient.candidate_allergens.len() == 1 {
                // This ingredient now has a definite match.
                matched_ingredients.insert(ingredient.name.clone());
                matches.insert(ingredient.candidate_allergens.iter().next().unwrap().clone(), ingredient.name.clone());
            }
        }
    }

    let mut allergen_names: Vec<&String> = allergens.iter().collect();
    allergen_names.sort();
    let dangerous_ingredients = allergen_names.iter().map(|&name| matches.get(name).unwrap().clone()).collect::<Vec<String>>().join(",");
    println!("Part 2: {}", dangerous_ingredients);

    (part1,0)
}

struct Ingredient {
    name: String,
    appears_in: HashSet<usize>,
    candidate_allergens: HashSet<String>,
}

impl Ingredient {
    fn new(name: String) -> Self {
        Self {
            name,
            appears_in: HashSet::new(),
            candidate_allergens: HashSet::new(),
        }
    }
}

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn parse(input_line: &str) -> Self {
        let ingredient_end = input_line.find('(').unwrap();
        let ingredients: HashSet<String> = input_line[0..ingredient_end].split(' ').map(str::to_string).filter(|name| !name.is_empty()).collect();
        let allergens: HashSet<String> = input_line[ingredient_end..].split(' ').skip(1).map(|allergen| {
            let mut allergen_string = allergen.to_string();
            allergen_string.truncate(allergen.len() - 1);
            allergen_string
        }).collect();
        Self { ingredients, allergens }
    }
}