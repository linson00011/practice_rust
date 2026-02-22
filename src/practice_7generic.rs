use std::ops::Add;

//1.trait: define intention,interface. use `trait` and `impl for` for define and impliment.
//         1.1 difine action ,eg:`trait TDoubleMe {fn double_me`
//         1.2 impliment  .   impl xxx for xxx, eg:`impl TDoubleMe for f32`
//                            you can impl it for system struct,eg i32 ,also you can for your struct.
//2.generic: extend fn or struct.
//         2.1 extend fn. eg:`fn fn_name <T:Add<Output = T>>`
//         2.2 extend struct .eg:`pub struct Point<T>`,
//           2.2.1 impl struct .eg `impl<T> Point<T>`
//           2.2.2 impl struct for one type. eg:`impl Point<f32>`
//3.difference:   3.1 just contain same intention,choose trait or generic function.
//                3.2 choose generic struct if they are a series objects.
//                    eg.Result<T>, both contain ok or err enum,just different content in it.
//                3.3 trait imp with different datatypes, so we can wrap trait with generic for pub and hide train.
pub fn practice_generic_fns() {
    //trait
    practice7_generic_trait();

    //generic funtion
    let num_result = practice7_generic_add(1, 2);
    println!("{}", num_result);

    let num_result = practice7_generic_add(1.1, 2.1);
    println!("{}", num_result);

    //generic struct
    let p1 = mod_point::Point { x: 3, y: 5 };
    let dis_p1 = p1.distance_from_origin();
    println!("x:{},dis:{}", p1.get_x(), dis_p1);

    let p2 = mod_point::Point { x: 3.0, y: 4.0 };
    let dis_p2 = p2.distance_from_origin();
    println!("x:{},dis:{}", p2.get_x(), dis_p2);

    test_lifetime();
}

/*#region lifetime */
fn test_lifetime() {
    let mut s1 = "apple".to_string();
    let s2 = "orange".to_string();

    let aa = test_ref_fn(&s1, &s2);

    let aa_1 = aa.to_string();
    s1.push('a');
    println!("aaaaaaaaaaaaaaaaaaaaaaa{}", aa_1);
}

fn test_ref_fn<'a>(s1: &'a str, _s2: &str) -> &'a str {
    s1
}
/*#endregion */
/* #region trait */
fn practice7_generic_trait() {
    let num_result = TDoubleMe::double_me(3);
    println!("{}", num_result);

    let num_result = TDoubleMe::double_me(3.1);
    println!("{}", num_result);

    let num_result = TDoubleMe::double_me("abc".to_string());
    println!("{}", num_result);
}
trait TDoubleMe {
    fn double_me(self) -> Self;
}

impl TDoubleMe for f32 {
    fn double_me(self) -> Self {
        (self) + (self)
    }
}

impl TDoubleMe for i32 {
    fn double_me(self) -> Self {
        (self) + (self)
    }
}

impl TDoubleMe for String {
    fn double_me(self) -> Self {
        format!("{}{}", self, self)
    }
}
/*#endregion */

/*#region generic function */
fn practice7_generic_add<T: Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}
/*#endregion */

/*#region generic struct */
mod mod_point {
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn get_x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        pub fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl Point<i32> {
        pub fn distance_from_origin(&self) -> i32 {
            ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt() as i32
        }
    }
}
/*#endregion */
