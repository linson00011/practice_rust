use std::collections::HashMap;

pub fn array_practice_string_hm() {
    let vec_split = array_practice_split_string("abc|efg");
    println!("{:?}", vec_split);

    let vec_split = array_practice_split_string(" ");
    let vec_join = vec_split.unwrap_or_else(|_| Vec::<&str>::new()).join("|");
    println!("test:{}", vec_join);

    let mut price_map = HashMap::<String, f32>::new();
    price_map.insert("apple".to_string(), 1.0);
    price_map.insert("orange".to_string(), 2.0);

    array_practice_string_hm_hashmap(&mut price_map);
    println!("{:?}", price_map);

    array_practice_string_hm_iter();
}

fn array_practice_string_hm_iter() {
    let data = "1|2|3|4|9".to_string();
    let rev_data = data.chars().rev().collect::<String>();
    println!("rev:{},index1:{:?}", rev_data, data.chars().nth(1));
    let remove_symbol = data.chars().filter(|x| (*x) != '|').collect::<String>();
    println!("remove |:{}", remove_symbol);
}

//hashmap: add .modify. remove
fn array_practice_string_hm_hashmap(price_map: &mut HashMap<String, f32>) {
    price_map.insert("megon".to_string(), 3.3);

    let the_apple = price_map.entry("apple".to_string()).or_insert(1.2);
    *the_apple += 1.2;

    price_map.remove("orange");
}

fn array_practice_split_string(data: &str) -> Result<Vec<&str>, anyhow::Error> {
    let rest = data
        .split('|')
        .filter(|x| !(*x).trim().is_empty())
        .collect::<Vec<&str>>();

    if rest.is_empty() {
        return Err(anyhow::Error::msg("empty"));
    } else {
        return Ok(rest);
    }
}

mod tests {

    #[test]
    fn split_empty() {
        let vec_split = crate::practice_4array_string_hashmap::array_practice_split_string("");
        assert!(vec_split.is_err());
    }
    #[test]
    fn split_empty2() {
        let vec_split = crate::practice_4array_string_hashmap::array_practice_split_string("||");
        assert!(vec_split.is_err());
    }

    #[test]
    fn split_normal() {
        //note: unwrap() will consume owership.
        let vec_split =
            crate::practice_4array_string_hashmap::array_practice_split_string("aa|b|c").unwrap();

        assert_eq!(vec_split[0], "aa");
        assert_eq!(vec_split[1], "b");
        assert_eq!(vec_split[2], "c");
    }
}
