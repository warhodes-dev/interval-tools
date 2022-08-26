use interval_tools::{
    Interval, 
    Endpoint,
    union,
};

#[test]
fn from_simple() {
    let a = Interval::from(0..5);
    assert_eq!(a.left, Endpoint::Closed(0));
    assert_eq!(a.right, Endpoint::Open(5));

    let b = Interval::from(..9);
    assert_eq!(b.left, Endpoint::Unbounded);
    assert_eq!(b.right, Endpoint::Open(9));

    let c = Interval::<i32>::from(..);
    assert_eq!(c.left, Endpoint::Unbounded);
    assert_eq!(c.right, Endpoint::Unbounded);
}

#[test]
fn overlapping() {
    let a = Interval::from(0..5);
    let ai = Interval::from(0..=5);
    let b = Interval::from(5..10);

    // [---)
    //     [---)
    assert!( !a.overlaps(&b) );
    // [---]
    //     [---)
    assert!( ai.overlaps(&b) );
}

#[test]
fn merging() {
    let a = Interval::from(0..5);
    let ai = Interval::from(0..=5);
    let b = Interval::from(5..10);

    let c = Interval::from(3..6);
    let d = Interval::from(5..);

    // [---)
    //     [---)
    assert!(union(&a, &b).is_none());
    // [---]
    //     [---)
    assert_eq!(union(&ai, &b).unwrap(), Interval::from(0..10));
    // [---)
    //   [-------->
    assert_eq!(union(&c, &d).unwrap(), Interval::from(3..));
}
