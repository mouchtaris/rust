mod fmap;
mod option;
mod rebind;
mod range;
mod flat_map;
mod pure_point;
mod traversable;
mod for_each;
mod element;
use ::rebind::Rebind;
use ::option::Optian;
use ::range::Range;
use ::fmap::FMap;
use ::flat_map::FlatMap;
use ::pure_point::PurePoint;
use ::traversable::Traversable;
use ::for_each::ForEach;
use ::element::Element;

impl Element for (i32, i32)
    where Self: Traversable
{
    type Element = i32;
}

impl Traversable for (i32, i32)
{
    fn next(&self) -> Optian<((i32, i32), i32)>
    {
        if self.0 > self.1 {
            return Optian::None;
        }
        return Optian::Some(((self.0 + 1, self.1), self.0));
    }
}





fn main() {
    let a = Optian::Some(12);
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

    let ti32: (i32, i32) = (12, 34);
    let pln = |x| println!("{:?}", x);
    ti32.for_each(pln);
    println!("Hello, world!");
}
