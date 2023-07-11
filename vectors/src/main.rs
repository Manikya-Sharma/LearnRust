fn main() {
    // declaring
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    // adding elements
    v2.push(5);
    v.push(3);
    // accessing
    let third: &i32 = &v2[2];
    println!("Third ELement: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("Value is {value}"),
        None => println!("There is no element"),
    }
    for i in &v2 {
        println!("{i}");
    }
    let mut v3 = vec![2, 3, 6];
    for i in &mut v3 {
        *i *= 2;
    }
    for i in &v3 {
        print!("{i}, ");
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
