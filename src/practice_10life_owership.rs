use practice_rust::printlns_simple;

pub fn practice_10owership_fns() {
    practice_string();
    practice_array();
}

/* 
=== 内存布局可视化（所有权转移后） ===
┌───────────────────────────────────────────┐
│ 栈（Stack, 0x7f开头）                     │
│ ┌───────────────────────────────────────┐ │
│ │ string_hi (失效，不可访问)            │ │
│ │ 原地址: 0x7fff06941d18                │ │
│ └───────────────────────────────────────┘ │
│ ┌───────────────────────────────────────┐ │
│ │ string_hi_ref (无变化)                │ │
│ │ 栈地址: 0x7fff06941d30, 指向: 0x7fff06941d18 │ │
│ └───────────────────────────────────────┘ │
│ ┌───────────────────────────────────────┐ │
│ │ string_hi2 (栈地址: 0x7fff06941e20)     │ │
│ │ 结构体内容：ptr=0x612b9001cd60, len=14, cap=16  │ │
│ └───────────────────────────────────────┘ │
└───────────────────────────────────────────┘
┌───────────────────────────────────────────┐
│ 堆（Heap, 0x61开头）                     │
│ ┌───────────────────────────────────────┐ │
│ │ 地址: 0x612b9001cd60（仅一份，所有权归string_hi2）│ │
│ │ 数据: "hi i am linson."                │ │
│ └───────────────────────────────────────┘ │
└───────────────────────────────────────────┘
*/
fn practice_string() {
    let string_hi = String::from("hi i am linson.");
    let string_hi_ref = &string_hi;
    println!("0x7f means it stores in stack. 0x5,0x6 means it stores in heap.");

    println!(
        "&string_hi{:p},string_hi.as_ptr(){:p},string_hi_ref{:p},&string_hi_ref{:p},string_hi_ref.as_ptr(){:p}",
        &string_hi, string_hi.as_ptr(), string_hi_ref, &string_hi_ref, string_hi_ref.as_ptr()
    );

    let string_hi2 = string_hi;
    println!("{:p},{:p}", &string_hi2, string_hi2.as_ptr());

    // can't use string_hi again. string_hi moves ownership to string_hi2 (only copies the stack pointer/len/cap, heap data is not copied)
    // println!("{:p}",&string_hi);
}

/* 
=== 第三步：切片创建后内存布局 ===
┌───────────────────────────────────────────┐
│ 栈（Stack, 0x7f开头）                     │
│ ┌───────────────────────────────────────┐ │
│ │ array_2 len:16字节=4×4 (栈地址: 0x7ffd8c7a9a00)               │ │
│ │ ┌──────┬──────┬──────┬──────┐          │ │
│ │ │  2   │  2   │  2   │  2   │          │ │
│ │ │0x7ffd8c7a9a00│0x7ffd8c7a9a04│0x7ffd8c7a9a08│0x7ffd8c7a9a0c│          │ │
│ │ └──────┴──────┴──────┴──────┘          │ │
│ ├───────────────────────────────────────┤ │
│ │ array_2_slice1,len:&[i32;4]，普通指针8字节, (&[i32], 栈地址: 0x7ffd8c7a9a10) │ │
│ │ 指向: 0x7ffd8c7a9a00, 长度: 4                   │ │
│ ├───────────────────────────────────────┤ │
│ │ array_2_slice2,len:胖指针（16 字节）:指针部分,长度部分 (&[i32], 栈地址: 0x7ffd8c7a9a18) │ │
│ │ 指向: 0x7ffd8c7a9a00, 长度: 2                   │ │
│ │ （仅覆盖array_2前2个元素）             │ │
│ └───────────────────────────────────────┘ │
└───────────────────────────────────────────┘
*/

fn practice_array() {
    let _array_1: [i32; 4]; // 预留内存空间,Rust 强制要求未初始化变量不能使用,访问
    // println!("{:p}",&array_1);

    // Rust 中固定大小的数组（[T; N]）是 “紧凑存储” 的，仅占用 size_of::<T>() * N 字节的栈空间，
    // 没有任何额外的元数据（如长度、指针等）开销，&array_2、&array_2[0]、array_2.as_ptr() 打印的地址也完全一致。
    let mut array_2 = [1; 4];
    let array_3 = [2; 4];
    println!("{:p},{:p},{:p},{}", &array_2, &array_2[0], array_2.as_ptr(), array_2[0]);
    println!("{:p},{:p},{:p},{}", &array_3, &array_3[0], array_3.as_ptr(), array_3[0]);

    // 数组[i32;4]实现了Copy trait（所有元素类型i32是Copy），赋值时拷贝所有元素值，而非转移所有权（数组也有所有权，只是Copy类型赋值不Move）
    array_2 = array_3;
    println!("{:p},{:p},{:p},{}", &array_2, &array_2[0], array_2.as_ptr(), array_2[0]);
    println!("{:p},{:p},{:p},{}", &array_3, &array_3[0], array_3.as_ptr(), array_3[0]);

    let array_2_slice1=&array_2;// &[i32;4] 固定数组引用，普通指针（8字节）
    let array_2_slice2=&array_2[0..2];//&[i32] 动态切片，胖指针（16字节）=指针部分+长度部分
    println!("{:p},{:p},{:p},{}", array_2_slice1, &array_2_slice1, array_2_slice1.as_ptr(), array_2_slice1[0]);
    println!("{:p},{:p},{:p},{}", array_2_slice2, &array_2_slice2, array_2_slice2.as_ptr(), array_2_slice2[0]);

    //node:array_2_slice1 is ref. but compile know it is &[i32;4]
    practice_array_get_v(array_2_slice1, array_2_slice2);
}


fn practice_array_get_v(a:&[i32;4],b:&[i32])
{
    printlns_simple!(a,b);
}


