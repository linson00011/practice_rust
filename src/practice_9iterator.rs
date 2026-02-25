use std::iter;

use practice_rust::printlns_simple;

pub fn practice_iterator_fns() {
    //1.what can be converted to an iterator? 2.how ? 3.what operations should an iterator have?

    //aq1:string and vec and ...;
    //aq2:inter:borrow. into_iter():owership.   iter_mut():mut ref
    //aq3:visit,filter,sum,modify.
    //    3.1 filter , 3.2 for_each, visit all elements
    //    3.3 map, visit all elements and  generate new iterator. eg,(|x|*x),  return *x to generate new iterator.
    //    3.3 sum,fold, collect,  they can output summary or collection.
    //    3.4 into_iter, need mofiy no copytype, eg:  ...into_iter(); ...map(|mut x|{...;x})

    //todo .flatten() flat. need to add an exsample.
    //dna.chars().enumerate().for_each(|(x,y)|{if error_index==-1 &&  !dna_code.contains(&y) {error_index=x as i32}   });

    let data_string = "hi,i am linson.".to_string();
    let iter_string = practice_iterater_convert_string(&data_string);
    

    let data_vec = vec![1, 2, 3, 4, 5, 6, 0];
    let iter_vec = practice_iterater_convert_vec(&data_vec);


    let data_vec_enum = vec![Some(1.2), Some(2.3), None, Some(5.4)];
    let iter_vec_enum = practice_iterater_convert_vec2(&data_vec_enum);

     //filter for_each
    iter_string.filter(|x|(*x)>'h').for_each(|x|println!("{:?}",x));
    //sum.
    let sum1:i32= iter_vec.clone().sum();
    println!("{}",sum1);
    //fold
    let sum2:i32=iter_vec.clone().fold(0, |temp,x|*x+temp);
    println!("{:?}",sum2);
     //collect ,map (modify or new iterator.) 
    let newVec:Vec<i32> = iter_vec.clone().map(|x|*x).collect();
     println!("{:?}",newVec);

     //map,collect. note: map consume iter and return new iterator.
     let sum3:i32=iter_vec.map(|x|x*x).sum();
     println!("{:?}",sum3);
    

    let data_vec_string=vec!["hi".to_string(),"ok".to_string()];
    let iter_vec_string=data_vec_string.into_iter();//note:need into.string is not a copy type.
    let tempstring:Vec<String>= iter_vec_string.map(|mut x|{x.push_str("!");x}).collect();
    printlns_simple!(tempstring);

    result_with_list();
    list_of_results();
}

fn practice_iterater_inspect()
{

}


fn practice_iterater_convert_string(data: &str) -> impl Iterator<Item = char>{
    data.chars()
}

//the item of return value is ref when data is vec ,even it contian i32.
fn practice_iterater_convert_vec(data: &Vec<i32>) -> impl Iterator<Item = &i32> +Clone {
    data.iter() //inter:borrow. into_iter():owership.   iter_mut():mut ref
}

fn practice_iterater_convert_vec2(data: &Vec<Option<f32>>) -> impl Iterator<Item = &Option<f32>> {
    data.iter()
}


#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b==0{
        Err(DivisionError::DivideByZero)
    }
    else if a==i64::MIN && b==-1 {
        Err(DivisionError::IntegerOverflow)
    }
    else if a %b!=0 {
        Err(DivisionError::NotDivisible)
    }
    else {
        Ok(a/b)
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list()->Result<[i64;4],DivisionError> {
   let numbers = [27, 297, 38502, 81];
   numbers.into_iter().for_each(|mut n| {n= divide(n, 27).unwrap();});
   Ok(numbers)
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() ->[Result<i64,DivisionError>;4]{
    let numbers = [27, 297, 38502, 81];
    [
        divide(numbers[0], 27),
        divide(numbers[1], 27),
        divide(numbers[2], 27),
        divide(numbers[3], 27),
    ]
}