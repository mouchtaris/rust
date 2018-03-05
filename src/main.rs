mod fmap;
mod option;
mod rebind;
mod range;
use ::fmap::*;
use ::option::*;
use ::range::*;






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
