pub fn my_exercims() {
    let _a = reverse("hi i am");
}

fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
