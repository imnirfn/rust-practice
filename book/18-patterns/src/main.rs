fn main() {
    // TODO
    // determine what favourite color is (using pattern matching)
    //
    //

    // ifelse
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: u8 = 34;

    if let Some(color) = fav_color {
        println!("using your fav color, {}, as the background", color)
    } else if is_tuesday {
        println!("tuesday is green day");
    } else if age > 30 {
        println!("using purple as the background");
    }
    
    // while / if loop
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    let v = vec!["v", "a", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (23, 22);
    print_coords(&point);

    // patterns
    let x = 'f';
    let y = 4;

    match x {
        'a'..='h' => println!("early ASCII letter"),
        'i'..='z' => println!("late ASCII letter"),
        _ => println!("none"),
    }
    match y {
        1 | 4 => println!("one or four"),
        2..=7 => println!("two till 7"),
        8 => println!("eight"),
        _ => println!("none"),
    }

    // destructuring
    let p = Point {
        x: 0,
        y: 10
    };

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(10, y);

    // destructuring struct
    match p {
        Point {x, y: 0} => println!("on the x axis at {}", x),
        Point {x: 0, y} => println!("on the y axis at {}", y),
        Point {x, y} => println!("on neither axis: ({}, {})", x, y),
    }


}

struct Point {
    x: i32,
    y: i32,
}

fn print_coords(&(x, y): &(i32, i32)) {
    println!("current locaion: {} {}", x, y);
}
