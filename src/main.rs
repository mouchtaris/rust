mod fmap;
mod option;
mod rebind;
use ::fmap::*;
use ::option::*;


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
    let a = Optian::Some::<i32>(12);
    let b = Optian::None::<i32>;
    let f: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    let am = a.fmap(f);
    let bm = b.fmap(f);

    let c = Optian::Some::<Range>(Range { from: 2, to: 8, step: 3 });
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
