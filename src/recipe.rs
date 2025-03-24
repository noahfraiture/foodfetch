use anyhow::Result;
use colored::Colorize;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::sync::Arc;
use std::{cmp::max, fmt};
use crate::ascii;
use crate::cli::Info;
use strsim::levenshtein;
use textwrap::{wrap, Options};
use once_cell::sync::Lazy;

const MEAL_CACHE_DATA: &str = include_str!("data/meal_cache.json");

#[derive(Serialize, Deserialize, Clone)]
struct MealCacheEntry {
    #[serde(rename = "idMeal")]
    id: String,
    #[serde(rename = "strMeal")]
    name: String,
    #[serde(rename = "strMealThumb")]
    thumb: String,
    #[serde(rename = "strInstructions")]
    instructions: Option<String>,
    #[serde(rename = "strCategory")]
    category: Option<String>,
    #[serde(rename = "strArea")]
    area: Option<String>,
    #[serde(rename = "strSource")]
    source: Option<String>,
    #[serde(rename = "strYoutube")]
    youtube: Option<String>,
    #[serde(rename = "strIngredient1")] ing1: Option<String>,
    #[serde(rename = "strIngredient2")] ing2: Option<String>,
    #[serde(rename = "strMeasure1")] meas1: Option<String>,
    #[serde(rename = "strMeasure2")] meas2: Option<String>,
}

impl MealCacheEntry {
    fn to_recipe(self) -> Recipe {
        Recipe {
            idMeal: Some(self.id),
            strMeal: Some(self.name),
            strMealThumb: Some(self.thumb),
            strInstructions: self.instructions,
            strCategory: self.category,
            strArea: self.area,
            strSource: self.source,
            strYoutube: self.youtube,
            strIngredient1: self.ing1,
            strIngredient2: self.ing2,
            strMeasure1: self.meas1,
            strMeasure2: self.meas2,
            ..Recipe::default()
        }
    }
}

static MEAL_CACHE: Lazy<Vec<MealCacheEntry>> = Lazy::new(|| {
    serde_json::from_str(MEAL_CACHE_DATA).unwrap_or_default()
});

fn capitalize_each_word(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn search_with_fuzzy(keyword: &str) -> Result<Recipes> {
    let original = keyword.trim();
    let lowercase = original.to_lowercase();
    let capitalized = capitalize_each_word(&lowercase);

    match Recipes::search(&lowercase).or_else(|_| Recipes::search(&capitalized)) {
        Ok(r) => return Ok(r),
        Err(_) => {}  
    }

    if let Some(best_match) = MEAL_CACHE.iter()
        .filter(|meal| {
            let distance = levenshtein(&meal.name.to_lowercase(), &lowercase);
            distance <= max(meal.name.len(), lowercase.len()) / 2
        })
        .min_by_key(|meal| levenshtein(&meal.name.to_lowercase(), &lowercase))
    {
        println!("⚠️  No exact match found for \"{}\".", original);
        println!("💡 Did you mean: \"{}\"? Using cached data...", best_match.name);
        
        return Ok(Recipes {
            meals: Some(vec![best_match.clone().to_recipe()])
        });
    }

    anyhow::bail!("❌ No recipes or close matches found for \"{}\".", original)
}

#[derive(Debug)]
pub struct DisplayRecipe {
    id: u32,
    title: String,
    category: String,
    area: String,
    ingredients: Vec<(String, String)>,
    instructions: String,
    tutorial_url: String,
    youtube_url: String,
    image_url: String,
    image_msg: Vec<String>,
    infos: Arc<Vec<Info>>,
}

impl fmt::Display for DisplayRecipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let show_instructions = self.infos.iter().any(|i| i == &Info::All || i == &Info::Instructions);
        let show_links = self.infos.iter().any(|i| i == &Info::All || i == &Info::Links);
        let show_image = self.infos.iter().any(|i| i == &Info::All);

        let mut image = if show_image { self.image_msg.clone() } else { Vec::new() };
        let mut start_lines: Vec<String> = Vec::new();
        let mut end_lines: Vec<String> = Vec::new();

        start_lines.push(format!("\t{} : {} ({})", "Title".red(), self.title, self.id));
        start_lines.push(format!("\t{}", "----".red()));
        if !self.category.is_empty() {
            start_lines.push(format!("\t{} : {}", "Category".red(), self.category));
        }
        if !self.area.is_empty() {
            start_lines.push(format!("\t{} : {}", "Area".red(), self.area));
        }
        start_lines.push(format!("\t{}", "Ingredients : ".red()));
        for (ingredient, quantity) in &self.ingredients {
            start_lines.push(format!("\t\t - {ing} ({qty})", ing = ingredient, qty = quantity));
        }

        if show_links {
            if !self.tutorial_url.is_empty() {
                end_lines.push(format!("\t{} :", "Tutorial".red()));
                end_lines.push(format!("\t {}", self.tutorial_url));
            }
            if !self.youtube_url.is_empty() {
                end_lines.push(format!("\t{} :", "Youtube".red()));
                end_lines.push(format!("\t {}", self.youtube_url));
            }
            if !self.image_url.is_empty() {
                end_lines.push(format!("\t{} :", "Image url".red()));
                end_lines.push(format!("\t {}", self.image_url));
            }
        }

        let needed_height = if show_image {
            max(start_lines.len() + end_lines.len() + 1, image.len().max(20))
        } else {
            start_lines.len() + end_lines.len() + 1
        };
        
        if image.len() < needed_height {
            image.resize(needed_height, String::new());
        }

        let needed_for_start = start_lines.len() + 1;
        if image.len() < needed_for_start {
            image.resize(needed_for_start, String::new());
        }
        for (index, line) in start_lines.iter().enumerate() {
            image[index + 1].push_str(line);
        }
        let start_of_end = max(image.len().saturating_sub(end_lines.len()), start_lines.len() + 1);
        let needed_for_end = start_of_end + end_lines.len();
        if image.len() < needed_for_end {
            image.resize(needed_for_end, String::new());
        }
        for (index, line) in end_lines.iter().enumerate() {
            image[index + start_of_end].push_str(line);
        }

        write!(f, "{}", image.join("\n"))?;

        if show_instructions && !self.instructions.is_empty() {
            write!(f, "\n\n\t{}\n", "Instructions : ".red())?;
            
            let options = Options::new(80)
                .initial_indent("\t ")
                .subsequent_indent("\t ");
                
            for line in wrap(&self.instructions, &options) {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

impl Recipe {
    pub fn to_display_recipe(self, infos: Arc<Vec<Info>>) -> DisplayRecipe {
        let show_image = infos.iter().any(|i| i == &Info::All);
        let image = if show_image && !self.strMealThumb.as_ref().unwrap_or(&String::new()).is_empty() {
            ascii::get_image(
                self.strMealThumb.as_ref().unwrap(),
                self.strMeal.as_ref().unwrap_or(&String::new())
            ).unwrap_or_default()
        } else {
            Vec::new()
        };
              
        DisplayRecipe {
            id: from_str::<u32>(self.idMeal.as_ref().unwrap_or(&String::new())).unwrap_or(0),
            title: self.strMeal.as_ref().unwrap_or(&String::new()).to_string(),
            category: self.strCategory.as_ref().unwrap_or(&String::new()).to_string(),
            area: self.strArea.as_ref().unwrap_or(&String::new()).to_string(),
            ingredients: self.get_ingredients(),
            instructions: self.strInstructions.as_ref().unwrap_or(&String::new()).to_string(),
            tutorial_url: self.strSource.as_ref().unwrap_or(&String::new()).to_string(),
            youtube_url: self.strYoutube.as_ref().unwrap_or(&String::new()).to_string(),
            image_url: self.strMealThumb.as_ref().unwrap_or(&String::new()).to_string(),
            image_msg: image,
            infos,
        }
    }

    fn get_ingredients(&self) -> Vec<(String, String)> {
        vec![
            (self.strIngredient1.clone(), self.strMeasure1.clone()),
            (self.strIngredient2.clone(), self.strMeasure2.clone()),
            (self.strIngredient3.clone(), self.strMeasure3.clone()),
            (self.strIngredient4.clone(), self.strMeasure4.clone()),
            (self.strIngredient5.clone(), self.strMeasure5.clone()),
            (self.strIngredient6.clone(), self.strMeasure6.clone()),
            (self.strIngredient7.clone(), self.strMeasure7.clone()),
            (self.strIngredient8.clone(), self.strMeasure8.clone()),
            (self.strIngredient9.clone(), self.strMeasure9.clone()),
            (self.strIngredient10.clone(), self.strMeasure10.clone()),
            (self.strIngredient11.clone(), self.strMeasure11.clone()),
            (self.strIngredient12.clone(), self.strMeasure12.clone()),
            (self.strIngredient13.clone(), self.strMeasure13.clone()),
            (self.strIngredient14.clone(), self.strMeasure14.clone()),
            (self.strIngredient15.clone(), self.strMeasure15.clone()),
            (self.strIngredient16.clone(), self.strMeasure16.clone()),
            (self.strIngredient17.clone(), self.strMeasure17.clone()),
            (self.strIngredient18.clone(), self.strMeasure18.clone()),
            (self.strIngredient19.clone(), self.strMeasure19.clone()),
            (self.strIngredient20.clone(), self.strMeasure20.clone()),
        ].into_iter()
            .filter_map(|(ingredient, quantity)| match (ingredient, quantity) {
                (Some(ing), Some(qty)) if !ing.is_empty() && !qty.is_empty() => Some((ing, qty)),
                _ => None,
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Recipes {
    pub meals: Option<Vec<Recipe>>,
}

impl Recipes {
    pub fn random() -> Result<Self> {
        Ok(get("https://www.themealdb.com/api/json/v1/1/random.php")?.json::<Recipes>()?)
    }
    pub fn search(keyword: &str) -> Result<Self> {
        let url = format!("https://www.themealdb.com/api/json/v1/1/search.php?s={keyword}");
        let response = get(url)?.json::<Recipes>()?;
        if response.meals.is_none() {
            anyhow::bail!("No meals found for keyword: {keyword}");
        }
        Ok(response)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Default)]
pub struct Recipe {
    idMeal: Option<String>,
    pub strMeal: Option<String>,
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
