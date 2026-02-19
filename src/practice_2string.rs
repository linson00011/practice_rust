pub fn str_practice() {
    let res = str_practice_join();
    println!("{res}");

    let res = str_practice_substring("hi, i am an apple.");
    println!("{res}");

    let res = str_practice_substring2("hi, i am an apple.");
    println!("ioob:{res}");

    let res = str_practice_substring3("hi, i am num1.3");
    println!("get digits:{res}");

    let res = str_practice_rev("hi, i am an apple.");
    println!("{res}");

    format();
}

fn str_practice_join() -> String {
    format!("{}{:.3}", "the number is:", 3.0)
}

fn str_practice_rev(source: &str) -> String {
    source.chars().rev().collect()
}

fn str_practice_substring(source: &str) -> String {
    //it is safe if ioob.
    source.chars().skip(24).take(4).collect()
}

fn str_practice_substring2(source: &str) -> &str {
    //str is a Bare Slice  &str is a  slice . it's unsafe if ioob.
    &source[0..0]
}

fn str_practice_substring3(source: &str) -> String {
    source.chars().filter(|x| (*x).is_numeric()).collect()
}

fn format() {
    println!("******foramt*********");
    print!("decimal:{:.3}.", 1.0);
    print!("left align:{:<8.3}.", 1.0);
    print!("right align:{:>8.3}.", 1.0);
    print!("bit:{:b}.", 3);
    print!("sign:{:+}.", 1);
    print!("0X:{:x}.", 254);
    print!("holdspace:{:05}.", 254);
    print!("e:{:e}.", 254);
    print!("split_str:{:.3}.", "hi,iam");
    print!("pointer:{:p}.", "hi,iam");
    println!("specail:{{.");
    println!("hi,{0:.2},{0}","linson");
}

mod tests{

    #[test]
    fn test() {
        crate::practice_2string::format();
    }
}