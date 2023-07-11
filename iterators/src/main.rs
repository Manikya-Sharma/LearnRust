// Iterator trait should be implemented
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {}", val);
    }
    let v2 = vec![2, 3, 4];
    let v3: Vec<_> = v2.iter().map(|x| x+2).collect();
}
