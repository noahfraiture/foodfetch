use std::thread;

use recipe::DisplayRecipe;

mod ascii;
mod cli;
mod recipe;

fn main() {
    let args = cli::args().unwrap();
    let recipes = if let Some(keyword) = args.keyword {
        recipe::Recipes::search(&keyword).unwrap()
    } else {
        recipe::Recipes::random().unwrap()
    };
    let handles: Vec<_> = recipes
        .meals
        .into_iter()
        .map(|meal| thread::spawn(move || meal.to_display_recipe(args.infos)))
        .collect();
    let display_recipes: Vec<DisplayRecipe> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    for display_recipe in &display_recipes {
        println!("{}\n", display_recipe);
    }
}
