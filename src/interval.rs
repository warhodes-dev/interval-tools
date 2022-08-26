/*use std::fmt::Debug;

/// The endpoints of an `Interval<T>`
///
/// Can be `Open(T)`, `Closed(T)`, or Unbounded
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Endpoint<T> {
    /// An open endpoint does not include the representative value (e.g. Open(3) does not include 3).
    Open(T),
    /// A closed endpoint does include the representative value (e.g. Closed(3) includes 3).
    Closed(T),
    /// An unbounded endpoint contains all values in that direction.
    Unbounded,
} 

/// Represents a single interval between two Endpoint<T>, expressed as the member fields left and right.
///
/// Interval values always ‘ascend’ from left to right, i.e. left <= right.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Interval<T> {
    pub left: Endpoint<T>,
    pub right: Endpoint<T>,
}

/* Type Implementations */

impl<T> Endpoint<T> {}

impl<T> Interval<T>
where T: Ord 
{
    pub fn overlaps(&self, other: &Interval<T>) -> bool {
        
        // Does X (x1, x2) overlap Y (y1, y2)?
        // In the fully exclusive case, A overlaps B if x1 < y2 && y1 < x2 .

        let x1 = &self.left;
        let x2 = &self.right;
        let y1 = &other.left;
        let y2 = &other.right;

        // First expression: x1 <(=) y2
        let lhs = match (x1, y2) {
            (Endpoint::Closed(x1), Endpoint::Closed(y2)) => { x1 <= y2 },
            (Endpoint::Closed(x1), Endpoint::Open(y2)) |
            (Endpoint::Open(x1), Endpoint::Closed(y2)) |
            (Endpoint::Open(x1), Endpoint::Open(y2)) => { x1 < y2 },
            (_,_) => true // Either point is unbounded
        };

        // Second expression: y1 <(=) x2
        let rhs = match (y1, x2) {
            (Endpoint::Closed(y1), Endpoint::Closed(x2)) => { y1 <= x2 },
            (Endpoint::Closed(y1), Endpoint::Open(x2)) |
            (Endpoint::Open(y1), Endpoint::Closed(x2)) |
            (Endpoint::Open(y1), Endpoint::Open(x2)) => { y1 < x2 },
            (_,_) => true
        };

        lhs && rhs
    }
}

/* Trait Implementations */

impl<T> From<(Endpoint<T>, Endpoint<T>)> for Interval<T> {
    fn from(pair: (Endpoint<T>, Endpoint<T>)) -> Interval<T> {
        Interval{ left: pair.0, right: pair.1 }
    }
}

impl<T> From<std::ops::Range<T>> for Interval<T> {
    fn from(r: std::ops::Range<T>) -> Interval<T> {
        let left = Endpoint::Closed(r.start);
        let right = Endpoint::Open(r.end);
        Interval{ left, right }
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for Interval<T> {
    fn from(r: std::ops::RangeInclusive<T>) -> Interval<T> {
        let inner = r.into_inner();
        let left = Endpoint::Closed(inner.0);
        let right = Endpoint::Closed(inner.1);
        Interval{ left, right }
    }
}

impl<T> From<std::ops::RangeTo<T>> for Interval<T> {
    fn from(r: std::ops::RangeTo<T>) -> Interval<T> {
        let left = Endpoint::Unbounded;
        let right = Endpoint::Open(r.end);
        Interval{ left, right }
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for Interval<T> {
    fn from(r: std::ops::RangeToInclusive<T>) -> Interval<T> {
        let left = Endpoint::Unbounded;
        let right = Endpoint::Closed(r.end);
        Interval{ left, right }
    }
}

impl<T> From<std::ops::RangeFrom<T>> for Interval<T> {
    fn from(r: std::ops::RangeFrom<T>) -> Interval<T> {
        let left = Endpoint::Closed(r.start);
        let right = Endpoint::Unbounded;
        Interval{ left, right }
    }
}

impl<T> From<std::ops::RangeFull> for Interval<T> {
    fn from(_r: std::ops::RangeFull) -> Interval<T> {
        let left = Endpoint::Unbounded;
        let right = Endpoint::Unbounded;
        Interval { left, right }
    }
}

/* Functions */

fn min_left<T>(a: &Interval<T>, b: &Interval<T>) -> Endpoint<T>
where
    T: Ord + Clone
{
    match (&a.left, &b.left) {
        (Endpoint::Closed(a), Endpoint::Open(b)) => {
            if a <= b { 
                Endpoint::Closed(a.clone()) 
            } else { 
                Endpoint::Open(b.clone()) 
            }
        } 
        (Endpoint::Open(a), Endpoint::Closed(b)) => {
            if a < b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Closed(a), Endpoint::Closed(b)) => {
            if a <= b {
                Endpoint::Closed(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Open(a), Endpoint::Open(b)) => {
            if a <= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Open(b.clone())
            }
        }
        (_,_) => Endpoint::Unbounded // Either point is unbounded
    }
}

fn min_right<T>(a: &Interval<T>, b: &Interval<T>) -> Endpoint<T>
where
    T: Ord + Clone
{
    match (&a.right, &b.right) {
        (Endpoint::Closed(a), Endpoint::Open(b)) => {
            if a < b { 
                Endpoint::Closed(a.clone()) 
            } else { 
                Endpoint::Open(b.clone()) 
            }
        } 
        (Endpoint::Open(a), Endpoint::Closed(b)) => {
            if a <= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Closed(a), Endpoint::Closed(b)) => {
            if a <= b {
                Endpoint::Closed(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Open(a), Endpoint::Open(b)) => {
            if a <= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Open(b.clone())
            }
        }
        (_,_) => Endpoint::Unbounded // Either point is unbounded
    }
}

fn max_left<T>(a: &Interval<T>, b: &Interval<T>) -> Endpoint<T>
where
    T: Ord + Clone
{
    match (&a.left, &b.right) {
        (Endpoint::Closed(a), Endpoint::Open(b)) => {
            if a > b { 
                Endpoint::Closed(a.clone())
            } else { 
                Endpoint::Open(b.clone())
            }
        } 
        (Endpoint::Open(a), Endpoint::Closed(b)) => {
            if a >= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Closed(a), Endpoint::Closed(b)) => {
            if a >= b {
                Endpoint::Closed(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Open(a), Endpoint::Open(b)) => {
            if a >= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Open(b.clone())
            }
        }
        (_,_) => Endpoint::Unbounded // Either point is unbounded
    }
}

fn max_right<T>(a: &Interval<T>, b: &Interval<T>) -> Endpoint<T>
where
    T: Ord + Clone
{
    match (&a.right, &b.right) {
        (Endpoint::Closed(a), Endpoint::Open(b)) => {
            if a >= b { 
                Endpoint::Closed(a.clone())
            } else { 
                Endpoint::Open(b.clone())
            }
        } 
        (Endpoint::Open(a), Endpoint::Closed(b)) => {
            if a > b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Closed(a), Endpoint::Closed(b)) => {
            if a >= b {
                Endpoint::Closed(a.clone())
            } else {
                Endpoint::Closed(b.clone())
            }
        }
        (Endpoint::Open(a), Endpoint::Open(b)) => {
            if a >= b {
                Endpoint::Open(a.clone())
            } else {
                Endpoint::Open(b.clone())
            }
        }
        (_,_) => Endpoint::Unbounded // Either point is unbounded
    }
}

/// Creates and returns an `Interval<T>` that represents the [union](https://en.wikipedia.org/wiki/Union_(set_theory))
/// of two overlapping intervals.
///
/// The union includes the contiguous entirety of both intervals.
///
/// Returns `None` if the intervals do not overlap.
pub fn union<T>(a: &Interval<T>, b: &Interval<T>) -> Option<Interval<T>>
where
    T: Ord + Clone
{
    if a.overlaps(b) {
        Some(Interval{ 
            left: min_left(a,b), 
            right: max_right(a,b),
        })
    } else {
        None
    }
}

/// Creates and returns an `Interval<T>` that represents the [intersection](https://en.wikipedia.org/wiki/Intersection_(set_theory))
/// of two intervals.
///
/// The intersection includes only the set of items that are shared by both intervals.
///
/// Returns `None` if the intervals do not overlap.
pub fn intersection<T>(a: &Interval<T>, b: &Interval<T>) -> Option<Interval<T>>
where
    T: Ord + Clone
{
    if a.overlaps(b) {
        Some(Interval{
            left: max_left(a,b),
            right: min_right(a,b),
        })
    } else {
        None
    }
}










*/
