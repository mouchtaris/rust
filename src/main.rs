mod fmap;
mod option;
mod rebind;
mod range;
mod flat_map;
use ::fmap::*;
use ::option::*;
use ::range::*;
use ::flat_map::*;


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

    let fatf: fn(i32) -> Optian<i32> = |x: i32| Optian::Some(x + 2);
    let afm = a.flat_map(fatf);

    println!("{:?}", a);
    println!("{:?}", am);
    println!("{:?}", b);
    println!("{:?}", bm);
    println!("{:?}", c);
    println!("{:?}", cm);
    println!("{:?}", cmm);
    println!("{:?}", cmmm);
    println!("{:?}", afm);
    println!("Hello, world!");
}
