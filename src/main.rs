use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::io::Write;
use std::process::{Command, Stdio};

mod ascii;

#[tokio::main]
async fn main() {
    let recipes = Recipes::random().await.unwrap();
    let display_recipes = recipes
        .meals
        .into_iter()
        .map(|r| r.to_display_recipe())
        .collect::<Vec<DisplayRecipe>>();
    println!("{:?}", display_recipes);
}

#[derive(Debug)]
struct DisplayRecipe {
    id: u32,
    title: String,
    category: String,
    area: String,
    ingredients: Vec<(String, String)>,
    tutorial: String,
    youtube: String,
    image_url: String,
}

impl Recipe {
    fn to_display_recipe(self) -> DisplayRecipe {
        let mut ingredients: Vec<(String, String)> = Vec::new();
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient1, self.strMeasure1) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient2, self.strMeasure2) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient3, self.strMeasure3) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient4, self.strMeasure4) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient5, self.strMeasure5) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient6, self.strMeasure6) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient7, self.strMeasure7) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient8, self.strMeasure8) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient9, self.strMeasure9) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient10, self.strMeasure10) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient11, self.strMeasure11) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient12, self.strMeasure12) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient13, self.strMeasure13) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient14, self.strMeasure14) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient15, self.strMeasure15) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient16, self.strMeasure16) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient17, self.strMeasure17) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient18, self.strMeasure18) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient19, self.strMeasure19) {
            ingredients.push((ingredient, quantity));
        };
        if let (Some(ingredient), Some(quantity)) = (self.strIngredient20, self.strMeasure20) {
            ingredients.push((ingredient, quantity));
        };
        let id = from_str::<u32>(&self.idMeal.unwrap_or_default()).unwrap_or(0);
        DisplayRecipe {
            id,
            title: self.strMeal.unwrap_or_default(),
            category: "".to_string(),
            area: self.strArea.unwrap_or_default(),
            ingredients,
            tutorial: self.strSource.unwrap_or_default(),
            youtube: self.strYoutube.unwrap_or_default(),
            image_url: self.strMealThumb.unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct Recipes {
    meals: Vec<Recipe>,
}

impl Recipes {
    async fn random() -> Result<Self> {
        Ok(
            reqwest::get("https://www.themealdb.com/api/json/v1/1/random.php")
                .await?
                .json::<Recipes>()
                .await?,
        )
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default)]
struct Recipe {
    idMeal: Option<String>,
    strMeal: Option<String>,
    strMealAlternate: Option<String>,
    strArea: Option<String>,
    strInstructions: Option<String>,
    strMealThumb: Option<String>,
    strTags: Option<String>,
    strYoutube: Option<String>,
    strIngredient1: Option<String>,
    strIngredient2: Option<String>,
    strIngredient3: Option<String>,
    strIngredient4: Option<String>,
    strIngredient5: Option<String>,
    strIngredient6: Option<String>,
    strIngredient7: Option<String>,
    strIngredient8: Option<String>,
    strIngredient9: Option<String>,
    strIngredient10: Option<String>,
    strIngredient11: Option<String>,
    strIngredient12: Option<String>,
    strIngredient13: Option<String>,
    strIngredient14: Option<String>,
    strIngredient15: Option<String>,
    strIngredient16: Option<String>,
    strIngredient17: Option<String>,
    strIngredient18: Option<String>,
    strIngredient19: Option<String>,
    strIngredient20: Option<String>,
    strMeasure1: Option<String>,
    strMeasure2: Option<String>,
    strMeasure3: Option<String>,
    strMeasure4: Option<String>,
    strMeasure5: Option<String>,
    strMeasure6: Option<String>,
    strMeasure7: Option<String>,
    strMeasure8: Option<String>,
    strMeasure9: Option<String>,
    strMeasure10: Option<String>,
    strMeasure11: Option<String>,
    strMeasure12: Option<String>,
    strMeasure13: Option<String>,
    strMeasure14: Option<String>,
    strMeasure15: Option<String>,
    strMeasure16: Option<String>,
    strMeasure17: Option<String>,
    strMeasure18: Option<String>,
    strMeasure19: Option<String>,
    strMeasure20: Option<String>,
    strSource: Option<String>,
    strImageSource: Option<String>,
    strCreativeCommonsConfirmed: Option<String>,
    dateModified: Option<String>,
}
