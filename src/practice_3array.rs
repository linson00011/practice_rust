pub fn array_practice() {
    //be care for the difference between difine virable,difine data type and operating variable when using mut,&,and &mut.
    let ret = array_practice_int_array([1, 2, 3, 4, 5]);
    println!("int array:{:?}", ret);

    let ret = array_practice_int_array2([1, 2, 3, 4, 5]);
    println!("int array:{:?}", ret);

    let vec_num = vec![1, 2, 3, 4, 5];
    let ret = array_practice_int_vec(vec_num);
    println!("int array:{:?}", ret);

    let mut vec_num2 = vec![1, 2, 3, 4, 5];
    array_practice_int_vec2(&mut vec_num2);
    println!("int array:{:?}", vec_num2);

    let vec_num3 = vec![1, 2, 3, 4, 5];
    let ret = array_practice_int_vec3(&vec_num3);
    println!("int array:{:?}", ret);

    //2 ways to borrow
    let vec_num4 = vec![1, 2, 3, 4, 5];
    let ref borrow_vec = vec_num4;
    let borrow_vec2 = &vec_num4;
    println!("{:?},{:?}", borrow_vec, borrow_vec2);
}

//iter:get &,need copied().   into_iter():retain owership.
fn array_practice_int_array(src: [i32; 5]) -> Vec<i32> {
    println!("len:{},index of 1:{}", src.len(), src[1]);
    src.iter().take(3).copied().collect()
}

fn array_practice_int_array2(src: [i32; 5]) -> Vec<i32> {
    //1.iter return &self, filter return &self  2. collect can't auto find the type each time if muliptle implicate exist.
    src.iter()
        .filter(|x| **x > 3)
        .copied()
        .collect::<Vec<i32>>()
}

//move owership
fn array_practice_int_vec(mut src: Vec<i32>) -> Result<Vec<i32>, anyhow::Error> {
    println!("len:{},index of 1:{}", src.len(), src[1]);
    src.push(6);
    src.push(7);
    src.push(8);
    let index_tobe_delete = 10;
    if src.len() > index_tobe_delete {
        src.remove(10);
    }
    let index_tobe_modify = 5;
    if src.len() > index_tobe_modify {
        src[5] = 0;
    }

    Ok(src)
}

//ref and enable mofify
fn array_practice_int_vec2(src: &mut Vec<i32>) {
    println!("len:{},index of 1:{}", (*src).len(), src[1]);
    (*src).push(6); //'src.push(6)' is simpler
    (*src).push(7);
    (*src).push(8);
    let index_tobe_delete = 10;
    if (*src).len() > index_tobe_delete {
        (*src).remove(10);
    }
    let index_tobe_modify = 5;
    if (*src).len() > index_tobe_modify {
        (*src)[5] = 0;
    }
}

//borrow
fn array_practice_int_vec3(src: &Vec<i32>) -> Result<Vec<i32>, anyhow::Error> {
    println!("len:{},index of 1:{}", src.len(), src[1]);
    let mut src = src.clone();
    src.push(6);
    src.push(7);
    src.push(8);
    let index_tobe_delete = 10;
    if src.len() > index_tobe_delete {
        src.remove(10);
    }
    let index_tobe_modify = 5;
    if src.len() > index_tobe_modify {
        src[5] = 0;
    }

    Ok(src)
}
