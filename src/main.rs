struct City {
    name: String,
    population: i64,
    country: String
}

// Cityをpopulation順に昇順ソートするためのヘルパー関数
fn city_popuration_descending(city: &City) -> i64 {
    -city.population
}

fn call_twice<F>(mut closure: F) where F: FnMut() {
    closure();
    closure();
}

use std::collections::HashMap;
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct BasicRouter<C> where C: Fn(&Request) -> Response {
    routes: HashMap<String, C>
}

fn main() {
    let mut cities = vec![City {name: "Tokyo".to_string(), population: 100, country: "Japan".to_string()},
                      City {name: "NY".to_string(), population: 99, country: "USA".to_string()}];

    cities.sort_by_key(city_popuration_descending);

    // ヘルパー関数の代わりにクロージャを使う
    cities.sort_by_key(|city| -city.population);

    // 関数にも型があり、変数に格納できる
    // fn(&City) -> i64はfn型（関数のみ）
    // Fn(&City) -> i64はFnトレイト（関数とクロージャの両方）
    let fn_city_popuration_descending: fn(&City) -> i64 = city_popuration_descending;
    cities.sort_by_key(fn_city_popuration_descending);

    let my_str = "hello".to_string();
    let f = || drop(my_str);
    f(); // メモリを解放してシステムに返却
    // 2回目の呼び出しはコンパイラに検出され「value used here after move」とエラーになる
    // f();

    // error: this closure implements `FnOnce`, not `Fn`
    // ジェネリック関数call_twiceはFnでなくFnOnceが実装されていると判断されコンパイルエラーになる
    // call_twice(f);

    // call_twiceの関数宣言でwhere制約をFnMut型にすると、データを更新するクロージャを呼び出してもコンパイルできる
    let mut i = 0;
    call_twice(|| i += 1);
    assert_eq!(i, 2);

    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);
}
