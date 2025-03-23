use std::{sync::Arc, thread};

use recipe::{DisplayRecipe, Recipes, search_with_fuzzy};

mod ascii;
mod cli;
mod recipe;

fn main() {
    let args = cli::args().unwrap();

    let recipes = if let Some(keyword) = args.keyword {
        match search_with_fuzzy(&keyword) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        Recipes::random().unwrap()
    };

    let meals = recipes.meals.unwrap_or_else(|| {
        eprintln!("⚠️  No meals found in the response.");
        std::process::exit(1);
    });

    let infos = Arc::new(args.infos);
    let handles: Vec<_> = meals
        .into_iter()
        .map(|meal| {
            let infos = infos.clone();
            thread::spawn(move || meal.to_display_recipe(infos))
        })
        .collect();

    let display_recipes: Vec<DisplayRecipe> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    for display_recipe in &display_recipes {
        println!("{}\n", display_recipe);
    }
}
