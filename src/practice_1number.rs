use practice_rust::printlns_simple;

pub fn num_practice() {
    let result = num_practice_add(1, 2);
    println!("1+2={}", result);

    let result = num_practice_division(1, 3);
    println!("1/3={:.20}", result);

    let result = num_practice_module(5, 3);
    println!("5 module 3:{}", result);

    num_pracice_sort();
    number_practice_quicksort();
}

fn num_pracice_sort() {
    let mut numbers = [1, 4, 9, 5, 6, 2, 2];
    for i in 0..numbers.len() - 1 {
        for j in 0..numbers.len() - 1 - i {
            if numbers[j] < numbers[j + 1] {
                let tempint = numbers[j + 1];
                numbers[j + 1] = numbers[j];
                numbers[j] = tempint;
            }
        }
    }
    printlns_simple!(numbers);
}

fn number_practice_quicksort() {
    let mut data = [5, 2, 9, 1, 5, 6, 7];
    //1.process array, split 2 arrays that keep left small, right bigger   2.recursion process those arrays.
    //3.so ,recursion logic, 3.1. exist when array 's len is 1. 3.2.process array, split 2 arrays that keep left small, right bigger 

    number_practice_quicksort_main(&mut data);
    printlns_simple!("quick sort:",data);

}

fn number_practice_quicksort_main(array:&mut [i32]){
    printlns_simple!("0.start:",array);
    //exist recurtion.
    let arraylent=array.len();
    if arraylent<=1{
        return;
    }
    //process array. split to 2 arrays. keep left small and right bigger. 3.recrution those 2 array.
    let compare_index=array.len()-1;
    let compare_value=array[compare_index];
    let mut small_index=0;

    for i in 0..=compare_index{
        if array[i]<=compare_value{
            array.swap(i, small_index);
            small_index+=1;
        }
    }

    //todo &mut,how explate it?
    {
        let small_area:&mut [i32]=&mut array[0..small_index-1];
        number_practice_quicksort_main(small_area);
    }
    let bigger_area:&mut [i32]=&mut array[small_index..arraylent];
    number_practice_quicksort_main(bigger_area);

}

fn num_practice_add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn num_practice_division(num1: i32, num2: i32) -> f64 {
    num1 as f64 / num2 as f64
}

fn num_practice_module(num1: i32, num2: i32) -> i32 {
    num1 % num2
}