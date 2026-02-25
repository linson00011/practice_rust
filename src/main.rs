use crate::{
    practice_2string::str_practice, practice_3array::array_practice,
    practice_6struct::practice_struct_fns,
};
use practice_1number::num_practice;

mod exercism;
mod practice_1number;
mod practice_2string;
mod practice_3array;
mod practice_4array_string_hashmap;
mod practice_5fn;
mod practice_6struct;
mod practice_7generic;
mod practice_8app_search_file;
mod practice_9iterator;

fn main() {
    let test_all = false;

    if test_all {
        num_practice();
        str_practice();
        exercism::my_exercims();
        array_practice();
        array_practice();
        practice_4array_string_hashmap::array_practice_string_hm();
        practice_5fn::practice_call_fns();
        practice_struct_fns();
        practice_7generic::practice_generic_fns();
        practice_8app_search_file::practice_app_search_file_main();
        practice_9iterator::practice_iterator_fns();
    }

   practice_8app_search_file::practice_app_search_file_main();
}

mod test {
    #[test]
    fn test_one() {
        crate::practice_3array::array_practice();
    }
}
