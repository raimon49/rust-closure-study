struct City {
    name: String,
    population: i64,
    country: String
}

// Cityをpopulation順に昇順ソートするためのヘルパー関数
fn city_popuration_descending(city: &City) -> i64 {
    -city.population
}

fn main() {
    let mut cities = vec![City {name: "Tokyo".to_string(), population: 100, country: "Japan".to_string()},
                      City {name: "NY".to_string(), population: 99, country: "USA".to_string()}];

    cities.sort_by_key(city_popuration_descending);
}
