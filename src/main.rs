mod fmap;
mod rebind;
use fmap::FMap;
use rebind::Rebind;


#[derive(Debug)]
enum Option<T> {
    Some(T),
    None
}

impl<T, U> Rebind<U> for Option<T> {
    type Element = T;
    type Type = Option<U>;
}

impl<T, U> FMap<U> for Option<T>
    where T: Copy
{
    fn fmap(&self, f: fn(T) -> U) -> Option<U> {
        match self {
            &Option::Some(t)=> Option::Some(f(t))   ,
            &Option::None   => Option::None
        }
    }
}



#[derive(Debug)]
struct Range {
    from    : i32,
    to      : i32,
    step    : i32,
}

impl std::clone::Clone for Range {
    fn clone(&self) -> Self {
        Range { .. *self }
    }
}
impl Copy for Range { }
impl Range {
    fn next(&self) -> Range { Range { from: self.from + self.step, .. *self } }
}



fn main() {
    let a = Option::Some::<i32>(12);
    let b = Option::None::<i32>;
    let f: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    let am = a.fmap(f);
    let bm = b.fmap(f);

    let c = Option::Some::<Range>(Range { from: 2, to: 8, step: 3 });
    let rnext: fn(Range) -> Range = |r: Range| r.next();
    let cm = c.fmap(rnext);
    let cmm = cm.fmap(rnext);
    let cmmm = cmm.fmap(rnext);

    println!("{:?}", a);
    println!("{:?}", am);
    println!("{:?}", b);
    println!("{:?}", bm);
    println!("{:?}", c);
    println!("{:?}", cm);
    println!("{:?}", cmm);
    println!("{:?}", cmmm);
    println!("Hello, world!");
}
