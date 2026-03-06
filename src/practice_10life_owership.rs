use std::{
    collections::{HashMap, hash_map},
    fmt::format,
    string,
};

use practice_rust::printlns_simple;

pub fn practice_10owership_fns() {
    let string_hi = "hi".to_string();
    let hi_refholder = ReferenceHolder::new(&string_hi);

    let hi2_ref_hoder: ReferenceHolder<i32>;
    {
        let int_1 = 32;
        hi2_ref_hoder = ReferenceHolder::new(&int_1);
    }

    let mut cache: HashMap<String, String> = HashMap::new();
    let ok1 = process_string("ok1", &mut cache);
    printlns_simple!(ok1);

    if let Some(result) = cache.get("ok1") {
        printlns_simple!(result);
    }

    let mut value_holder: ValusHolder<String> = ValusHolder::new();
    value_holder.process_string("valueHolder1");
    printlns_simple!(value_holder.data);

    value_holder.process_string_take_owership("valueHolder2".to_string());
    printlns_simple!(value_holder.data);
}

struct ValusHolder<T> {
    pub data: HashMap<String, T>,
}

impl<T> ValusHolder<T> {
    fn new() -> Self {
        ValusHolder {
            data: HashMap::<String, T>::new(),
        }
    }
}

impl ValusHolder<String> {
    fn process_string(&mut self, query: &str) -> Option<&str> {
        if !self.data.contains_key(query) {
            self.data.insert(query.to_string(), format!("{}:ok", query));
        }

        self.data.get(query).map(|x| x.as_str())
    }
    fn process_string_take_owership(&mut self, query: String) -> Option<&str> {
        if !self.data.contains_key(&query) {
            let ss = format!("{}:ok", query);
            self.data.insert(query.to_string(), ss);
        }

        self.data.get(&query).map(|x| x.as_str())
    }
}

//set query to &str because it will be created to string for hashmap.
fn process_string<'a>(query: &str, cache: &'a mut HashMap<String, String>) -> Option<&'a str> {
    if !cache.contains_key(query) {
        cache.insert(query.to_string(), format!("{}:ok", query));
    }

    cache.get(query).map(|x| x.as_str())
    //&cache[query]
}

//introduce life to `data` variable.
struct ReferenceHolder<'a, T> {
    data: &'a T,
}

impl<'a, T> ReferenceHolder<'a, T> {
    fn new(data: &'a T) -> Self {
        ReferenceHolder { data: data }
    }

    fn get(&self) -> &'a T {
        self.data
    }
}
