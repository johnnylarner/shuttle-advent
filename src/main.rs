use axum::{
    extract::{Json, Path},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

#[derive(serde::Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
    speed: f64,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten: u32,
}

#[derive(serde::Serialize)]
struct ReindeerComparisonResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

struct ReindeerComparisonResponseBuilder {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ReindeerComparisonResponseBuilder {
    fn new() -> Self {
        ReindeerComparisonResponseBuilder {
            fastest: String::new(),
            tallest: String::new(),
            magician: String::new(),
            consumer: String::new(),
        }
    }
    fn add_fastest(mut self, name: &str, strength: u32) -> Self {
        self.fastest = format!(
            "Speeding past the finish line with a strength of {:?} is {}",
            strength, name
        );
        self
    }
    fn add_tallest(mut self, name: &str, antler_width: u32) -> Self {
        self.tallest = format!(
            "{} is standing tall with his {:?} cm wide antlers",
            name, antler_width
        );
        self
    }
    fn add_magician(mut self, name: &str, snow_magic_power: u32) -> Self {
        self.magician = format!(
            "{} could blast you away with a snow magic power of {:?}",
            name, snow_magic_power
        );
        self
    }
    fn add_consumer(mut self, name: &str, favorite_food: &str) -> Self {
        self.consumer = format!(
            "{} ate lots of candies, but also some {}",
            name, favorite_food
        );
        self
    }

    fn build(self) -> ReindeerComparisonResponse {
        ReindeerComparisonResponse {
            fastest: self.fastest,
            tallest: self.tallest,
            magician: self.magician,
            consumer: self.consumer,
        }
    }
}

#[derive(serde::Serialize)]
struct ElfCount {
    elf: u32,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    shelf: u32,
}

impl ElfCount {
    fn from_elf_string(elf_string: &str) -> Self {
        let elf_identifier = "elf";
        let num_elves = elf_string.matches(elf_identifier).count();

        let elf_on_shelf_identifier = "elf on a shelf";
        let num_elves_on_shelves = elf_string.matches(elf_on_shelf_identifier).count();

        let shelf_without_elf_regex = "shelf";
        let num_shelves_without_elves = elf_string
            .match_indices(shelf_without_elf_regex)
            .filter(|m| {
                let (i, _) = m;
                let elf_on_substring = "elf on a ";
                let not_proceeded_by_elf = *i <= elf_on_substring.len()
                    || &elf_string[i - elf_on_substring.len()..*i] != elf_on_substring;

                not_proceeded_by_elf
            })
            .count();

        ElfCount {
            elf: num_elves as u32,
            elf_on_a_shelf: num_elves_on_shelves as u32,
            shelf: num_shelves_without_elves as u32,
        }
    }
}

async fn count_elves(elf_string: String) -> Json<ElfCount> {
    Json(ElfCount::from_elf_string(&elf_string))
}

async fn calc_reindeer_strength(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
    let total_strength: u32 = reindeers.into_iter().map(|r| r.strength).sum();
    total_strength.to_string()
}

async fn compare_reindeers(
    Json(reindeers): Json<Vec<Reindeer>>,
) -> Json<ReindeerComparisonResponse> {
    let builder = ReindeerComparisonResponseBuilder::new();

    let only_one_winner_possible = reindeers.len() == 1;
    if only_one_winner_possible {
        let winner = reindeers.first().unwrap();
        return Json(
            builder
                .add_fastest(&winner.name, winner.strength)
                .add_tallest(&winner.name, winner.antler_width)
                .add_magician(&winner.name, winner.snow_magic_power)
                .add_consumer(&winner.name, &winner.favorite_food)
                .build(),
        );
    }
    let fastest_speed = reindeers
        .iter()
        .map(|r| r.speed)
        .fold(0.0 as f64, |a, b| a.max(b));
    let fastest = reindeers
        .iter()
        .filter(|r| r.speed == fastest_speed)
        .next()
        .unwrap();
    let tallest = reindeers
        .iter()
        .max_by_key(|r| r.height + r.antler_width)
        .unwrap();
    let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let consumer = reindeers.iter().max_by_key(|r| r.candies_eaten).unwrap();

    Json(
        builder
            .add_fastest(&fastest.name, fastest.strength)
            .add_tallest(&tallest.name, tallest.antler_width)
            .add_magician(&magician.name, magician.snow_magic_power)
            .add_consumer(&consumer.name, &consumer.favorite_food)
            .build(),
    )
}

async fn exclusive_cube(Path(path): Path<String>) -> impl IntoResponse {
    let mut all_nums: Vec<u32> = Vec::new();

    for candidate_num in path.split("/") {
        let validated_num = match candidate_num.parse::<u32>() {
            Ok(is_digit) => is_digit,
            Err(_) => panic!("package id must be a digit"),
        };
        all_nums.push(validated_num)
    }

    let max_num_sledges = 20;
    if all_nums.len() > max_num_sledges {
        panic!("maximum of 20 sledges can be processed simultaneously")
    }

    if let Some(exclusive_or) = all_nums.into_iter().reduce(|a, b| a ^ b) {
        exclusive_or.pow(3).to_string()
    } else {
        panic!("no numbers provided")
    }
}
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/get", get(hello_world))
        .route("/1/*path", get(exclusive_cube))
        .route("/4/strength", post(calc_reindeer_strength))
        .route("/4/contest", post(compare_reindeers))
        .route("/6", post(count_elves));

    Ok(router.into())
}
