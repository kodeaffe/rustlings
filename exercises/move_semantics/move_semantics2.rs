// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let vec0: Vec<i32> = Vec::new();

    let mut vec1 = fill_vec(&vec0);
    //let mut vec1 = fill_vec(Vec::new());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec1: Vec<i32> = Vec::new();
    for item in vec {
        vec1.push(*item);
    }
    vec1.push(22);
    vec1.push(44);
    vec1.push(66);

    vec1
}
