fn main() {
    let mut i = 100;

    i = repo_A::add(1, i);
    i = repo_A::sub(i, 5);
    // i = repo_B::div(i, 2);
    // i = repo_B::mul(i, 3);

    println!("{i}");

    // repo_A::foo();
}
