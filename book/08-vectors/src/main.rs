fn main() {
    // explicitly declare the vectors
    let v: Vec<i32> = Vec::new();

    let v_inferred = vec![1, 2, 3];

    let mut v_mutable = Vec::new();

    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // reading vectors content
    
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third element is: {}", third),
        None => println!("Theres no third element"),
    }

    // looping over vectors (not mutable)
    for i in &v {
        println!("{}", i);
    }

    // looping over vectors & changing values (mutable)
    for i in &mut v_mutable {
        // * is a deference operator to get value in i
        // before using += operatot
        *i += 50;
        println!("mut {}", i);
    }
    vector_of_enums();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_of_enums() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.22),
        SpreadsheetCell::Text(String::from("Green")),
    ];

    for i in &row {
        println!("{:#?}", i);
    }
}
