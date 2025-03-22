use anyhow::Result;
use colored::Colorize;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{cmp::max, fmt};

use crate::ascii;
use crate::cli;
use crate::cli::Infos;

#[derive(Debug)]
pub struct DisplayRecipe {
    id: u32,
    title: String,
    category: String,
    area: String,
    ingredients: Vec<(String, String)>,
    tutorial: String,
    youtube: String,
    image_url: String,
    image_msg: Vec<String>,
    infos: cli::Infos,
}

impl fmt::Display for DisplayRecipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut image = self.image_msg.clone();
        let mut start_lines: Vec<String> = Vec::new();
        let mut end_lines: Vec<String> = Vec::new();
        start_lines.push(format!(
            "\t{} : {} ({})",
            "Title".red(),
            self.title,
            self.id
        ));
        start_lines.push(format!("\t{}", "----".red()));
        if !self.category.is_empty() {
            start_lines.push(format!("\t{} : {}", "Category".red(), self.category));
        }
        if !self.area.is_empty() {
            start_lines.push(format!("\t{} : {}", "Area".red(), self.area));
        }
        start_lines.push(format!("\t{}", "Ingredients : ".red()));
        for (ingredient, quantity) in &self.ingredients {
            start_lines.push(format!("\t\t - {ingredient} ({quantity})"));
        }
        if self.infos == cli::Infos::All && !self.tutorial.is_empty() {
            end_lines.push(format!("\t{} :", "Tutorial".red()));
            end_lines.push(format!("\t {}", self.tutorial));
        }
        if self.infos == cli::Infos::All && !self.youtube.is_empty() {
            end_lines.push(format!("\t{} :", "Youtube".red()));
            end_lines.push(format!("\t {}", self.youtube));
        }
        if self.infos == cli::Infos::All && !self.image_url.is_empty() {
            end_lines.push(format!("\t{} :", "Image url".red()));
            end_lines.push(format!("\t {}", self.image_url));
        }

        for (index, line) in start_lines.iter().enumerate() {
            if image.len() <= index + 1 {
                break;
            }
            image[index + 1].push_str(line);
        }
        let start_of_end = max(image.len() - end_lines.len(), start_lines.len() + 1);
        for (index, line) in end_lines.iter().enumerate() {
            if image.len() <= index + start_of_end {
                break;
            }
            image[index + start_of_end].push_str(line);
        }
        let image = image.join("\n");
        write!(f, "{image}")
    }
}

impl Recipe {
    pub fn to_display_recipe(self, infos: Infos) -> DisplayRecipe {
        let ingredients = vec![
            (self.strIngredient1, self.strMeasure1),
            (self.strIngredient2, self.strMeasure2),
            (self.strIngredient3, self.strMeasure3),
            (self.strIngredient4, self.strMeasure4),
            (self.strIngredient5, self.strMeasure5),
            (self.strIngredient6, self.strMeasure6),
            (self.strIngredient7, self.strMeasure7),
            (self.strIngredient8, self.strMeasure8),
            (self.strIngredient9, self.strMeasure9),
            (self.strIngredient10, self.strMeasure10),
            (self.strIngredient11, self.strMeasure11),
            (self.strIngredient12, self.strMeasure12),
            (self.strIngredient13, self.strMeasure13),
            (self.strIngredient14, self.strMeasure14),
            (self.strIngredient15, self.strMeasure15),
            (self.strIngredient16, self.strMeasure16),
            (self.strIngredient17, self.strMeasure17),
            (self.strIngredient18, self.strMeasure18),
            (self.strIngredient19, self.strMeasure19),
            (self.strIngredient20, self.strMeasure20),
        ];
        let ingredients = ingredients
            .into_iter()
            .filter_map(|(ingredient, quantity)| match (ingredient, quantity) {
                (Some(ing), Some(qty)) if !ing.is_empty() && !qty.is_empty() => Some((ing, qty)),
                _ => None,
            })
            .collect::<Vec<(String, String)>>();
        let id = from_str::<u32>(&self.idMeal.unwrap_or_default()).unwrap_or(0);
        let title = self.strMeal.unwrap_or_default();
        let category = self.strCategory.unwrap_or_default();
        let area = self.strArea.unwrap_or_default();
        let tutorial = self.strSource.unwrap_or_default();
        let youtube = self.strYoutube.unwrap_or_default();
        let image_url = self.strMealThumb.unwrap_or_default();

        let longest_text = [&title, &category, &area, &tutorial, &youtube, &image_url]
            .iter()
            .fold("", |old, new| if old.len() > new.len() { old } else { new });
        let image = if image_url.is_empty() {
            Vec::default()
        } else {
            ascii::get_image(&image_url, longest_text).unwrap_or_default()
        };
        DisplayRecipe {
            id,
            title,
            category,
            area,
            ingredients,
            tutorial,
            youtube,
            image_url,
            image_msg: image,
            infos,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Recipes {
    pub meals: Vec<Recipe>,
}

impl Recipes {
    pub fn random() -> Result<Self> {
        Ok(get("https://www.themealdb.com/api/json/v1/1/random.php")?.json::<Recipes>()?)
    }

    pub fn search(keyword: &str) -> Result<Self> {
        let url = format!("https://www.themealdb.com/api/json/v1/1/search.php?s={keyword}");
        Ok(get(url)?.json::<Recipes>()?)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default)]
pub struct Recipe {
    idMeal: Option<String>,
    strMeal: Option<String>,
    strMealAlternate: Option<String>,
    strCategory: Option<String>,
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
