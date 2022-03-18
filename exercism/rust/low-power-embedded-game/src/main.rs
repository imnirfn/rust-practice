fn main() {
    println!("testing for iter");
    let v = [0, 1, 2, 3, 4];

    let iter = v.iter();
    
    // next will make the iterator advances
    // assert_eq!(iter.next(), Some((0, &1)));
    // assert_eq!(iter.next(), Some((1, &2)));


    let x = iter.filter(|&x| *x > 1);
    println!("{:?}", x);


    let mut s = v.iter().step_by(2);
    assert_eq!(s.next(), Some(&0));
    assert_eq!(s.next(), Some(&1));
}
