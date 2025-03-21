use std::collections::{HashSet, HashMap};
struct Solution;

impl Solution {
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let recipes: HashMap<String, Vec<String>> = HashMap::from_iter(recipes.into_iter().zip(ingredients));
        let mut supplies: HashSet<String> = HashSet::from_iter(supplies);
        let mut available_recipes = HashSet::new();
        let mut has_changed = true;
        while has_changed {
            has_changed = false;
            for (recipe, ingredientes) in recipes.iter() {
                if available_recipes.contains(recipe) {
                    continue;
                }
                if ingredientes.iter().all(|x| supplies.contains(x)) {
                    available_recipes.insert(recipe.clone());
                    supplies.insert(recipe.clone());
                    has_changed = true
                }
            }
        }
        available_recipes.into_iter().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_recipes_example1() {
        let recipes = vec!["bread".to_string()];
        let ingredients = vec![vec!["yeast".to_string(), "flour".to_string()]];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "corn".to_string()];
        let result= Solution::find_all_recipes(recipes, ingredients, supplies);
        assert_eq!(result, vec!["bread".to_string()]);
    }

    #[test]
    fn test_find_all_recipes_example2() {
        let recipes = vec!["bread".to_string(), "sandwich".to_string()];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        let expected_result: HashSet<String>= HashSet::from(["bread".to_string(), "sandwich".to_string()]);
        assert_eq!(result.len(), expected_result.len());
        for recipe in result {
            assert!(expected_result.contains(&recipe));
        }
    }

    #[test]
    fn test_find_all_recipes_example3() {
        let recipes = vec![
            "bread".to_string(),
            "sandwich".to_string(),
            "burger".to_string(),
        ];
        let ingredients = vec![
            vec!["yeast".to_string(), "flour".to_string()],
            vec!["bread".to_string(), "meat".to_string()],
            vec!["sandwich".to_string(), "meat".to_string(), "bread".to_string()],
        ];
        let supplies = vec!["yeast".to_string(), "flour".to_string(), "meat".to_string()];
        let result = Solution::find_all_recipes(recipes, ingredients, supplies);
        let expected_result: HashSet<String>= HashSet::from(["bread".to_string(), "sandwich".to_string(), "burger".to_string()]);
        assert_eq!(result.len(), expected_result.len());
        for recipe in result {
            assert!(expected_result.contains(&recipe));
        }
    }
}
