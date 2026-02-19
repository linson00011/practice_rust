use core::str;

//1.callback: fn fn_name(handleshort:fn(&str)->String)
//2.trait:    pub trait trait_name{fns}
//3.box:      Vec::<Box<dyn mod_good::TGood>>::new()
pub fn practice_struct_fns() {
    practice_struct_init();

    let res = proceed_string("111", putstring, cutstring);
    println!("{}", res);

    let res = proceed_string("1115435646567687980654634", putstring, cutstring);
    println!("{}", res);
}

fn proceed_string(
    src: &str,
    handleshort: fn(&str) -> String,
    handlelong: fn(&str) -> String,
) -> String {
    if src.chars().count() > 10 {
        handlelong(src)
    } else {
        handleshort(src)
    }
}

fn putstring(src: &str) -> String {
    src.to_string() + "abc"
}

fn cutstring(src: &str) -> String {
    src.chars().take(10).collect()
}

mod mod_good {
    #[derive(Debug)]
    struct Good {
        name: String,
        price: f32,
    }

    impl Good {
        fn show_info(&self) -> String {
            format!("name:{},price:{:.2} ", self.name, self.price)
        }
    }

    //interface
    pub trait TGood {
        fn common_show(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Book {
        base: Good,
        publisher: String,
    }

    impl Book {
        pub fn new(name: &str, price: f32, publisher: &str) -> Result<Self, anyhow::Error> {
            //1.check data valid
            let mut valid = true;
            if name.trim().is_empty() || publisher.trim().is_empty() || price <= 0.0 {
                valid = false;
            }
            let ss = Self {
                base: Good {
                    name: name.to_string(),
                    price,
                },
                publisher: publisher.to_string(),
            };
            match valid {
                true => Ok(ss),
                false => Err(anyhow::Error::msg("invalid data.")),
            }
        }

        pub fn show_info(&self) -> String {
            format!("{}publisher:{}", self.base.show_info(), self.publisher)
        }
    }

    impl TGood for Book {
        fn common_show(&self) -> String {
            format!(
                "name:{},price:{:.2}  other info:{}",
                self.base.name, self.base.price, self.publisher
            )
        }
    }

    #[derive(Debug)]
    pub struct Fruit {
        base: Good,
        wholesaler: String,
    }

    impl Fruit {
        pub fn new(name: &str, price: f32, wholesaler: &str) -> Result<Self, anyhow::Error> {
            let mut valid = true;
            if name.trim().is_empty() || wholesaler.trim().is_empty() || price <= 0.0 {
                valid = false;
            }

            if valid {
                let ss = Self {
                    base: Good {
                        name: name.to_string(),
                        price,
                    },
                    wholesaler: wholesaler.to_string(),
                };
                Ok(ss)
            } else {
                Err(anyhow::Error::msg("invalid data."))
            }
        }

        pub fn show_info(&self) -> String {
            format!("{}wholesaler:{}", self.base.show_info(), self.wholesaler)
        }
    }

    impl TGood for Fruit {
        fn common_show(&self) -> String {
            format!(
                "name:{},price:{:.2}  other info:{}",
                self.base.name, self.base.price, self.wholesaler
            )
        }
    }
}

fn practice_struct_init() {
    //noted:
    let mut my_goods = Vec::<Box<dyn mod_good::TGood>>::new();

    let c_sharp = mod_good::Book::new("c#", 20.4, "people pp");
    match c_sharp {
        Ok(x) => {
            println!("{}", x.show_info());
            my_goods.push(Box::new(x));
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    let apple_product = mod_good::Fruit::new("apple", 2.0, "big ss");
    match apple_product {
        Ok(x) => {
            println!("{}", x.show_info());
            my_goods.push(Box::new(x));
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    //for consumes owership,use symbol &.
    for the_good in &my_goods {
        println!("common info:{}", the_good.common_show())
    }
}
