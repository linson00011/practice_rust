pub fn practice_call_fns() {
    let res = practice_call_fns_number_process(EnumNumberprocess::ADD, 1, 2);
    println!("{:?}", res);

    let res = practice_call_fns_number_process(EnumNumberprocess::MINES, 1, 2);
    println!("{:?}", res);

    let res = practice_call_fns_number_process(EnumNumberprocess::MULIPTLE, 1, 2);
    println!("{:?}", res);

    let res = practice_call_fns_number_process(EnumNumberprocess::DIVIDE, 1, 2);
    println!("{:?}", res);

    practice_fn_control();

    let res = practice_fn_string_plus("123", 10);
    println!("{:?}", res);

    let res = practice_fn_string_plus("1234567890", 10);
    println!("{:?}", res);

    let res = practice_fn_string_plus("12345678901", 10);
    println!("{:?}", res);

    let res = practice_fn_more_return_values();
    println!("{:?}", res);
}

enum EnumNumberprocess {
    ADD,
    DIVIDE,
    MULIPTLE,
    MINES,
}

fn practice_fn_more_return_values() -> (i32, f32, String) {
    (1, 1.1, "apple".to_string())
}

fn practice_fn_control() {
    let loop_total = 10;
    let mut num_result = 0;
    for i in 0..loop_total {
        num_result += i + 1;
    }
    println!("{}", num_result);
}

fn practice_fn_string_plus(src: &str, max_len: u32) -> String {
    let str_len = src.len() as u32;
    if str_len > max_len {
        let tempstr = src.chars().take(max_len as usize).collect::<String>();
        format!("{}...", tempstr)
    } else {
        src.to_string()
    }
}

fn practice_call_fns_number_process(
    op: EnumNumberprocess,
    num1: i32,
    num2: i32,
) -> Result<f32, anyhow::Error> {
    match op {
        EnumNumberprocess::ADD => Ok((num1 + num2) as f32),
        EnumNumberprocess::MINES => Ok((num1 - num2) as f32),
        EnumNumberprocess::MULIPTLE => Ok((num1 * num2) as f32),
        EnumNumberprocess::DIVIDE => {
            if num2 == 0 {
                Err(anyhow::Error::msg("error:zero"))
            } else {
                Ok(num1 as f32 / num2 as f32)
            }
        }
    }
}

mod tests {

    #[test]
    fn testdivide() {
        let zero_result = crate::practice_5fn::practice_call_fns_number_process(
            super::EnumNumberprocess::DIVIDE,
            2,
            0,
        );
        assert!(zero_result.is_err());

        let zero_result = crate::practice_5fn::practice_call_fns_number_process(
            super::EnumNumberprocess::DIVIDE,
            1,
            2,
        );
        assert_eq!(zero_result.unwrap(), 0.5);

        let result = crate::practice_5fn::practice_call_fns_number_process(
            super::EnumNumberprocess::DIVIDE,
            1,
            3,
        );
        let result = format!("{:.3}", result.expect("error"));
        assert_eq!(result, "0.333".to_string());
    }
}
