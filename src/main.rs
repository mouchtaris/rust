trait Rebind<U> {
    type Element;
    type Type;
}

trait FMap<U>
    where Self: Rebind<U>
{
    fn fmap(&self, f: fn(&Self::Element) -> U) -> Self::Type;
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None
}

impl<T, U> Rebind<U> for Option<T> {
    type Element = T;
    type Type = Option<U>;
}

impl<T, U> FMap<U> for Option<T> {
    fn fmap(&self, f: fn(&T) -> U) -> Option<U> {
        match self {
            &Option::Some(ref t)    => Option::Some(f(t))   ,
            &Option::None           => Option::None
        }
    }
}



struct Range {
    from: i32,
    to: i32,
    step: i32,
}

impl std::clone::Clone for Range {
    fn clone(&self) -> Self {
        Range {
            from: self.from,
            to: self.to,
            step: self.step,
        }
    }
}
impl Copy for Range { }
impl Range {
    fn next(&self) -> Range { Range { from: self.from + self.step, .. *self } }
}



fn main() {
    let a = Option::Some::<i32>(12);
    let b = Option::None::<i32>;
    let f: fn(&i32) -> i32 = |x: &i32| -> i32 { x + 1 };
    let am = a.fmap(f);
    let bm = b.fmap(f);
    println!("{:?}", a);
    println!("{:?}", am);
    println!("{:?}", b);
    println!("{:?}", bm);
    println!("Hello, world!");
}
