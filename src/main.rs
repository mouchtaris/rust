mod fmap;
mod option;
mod rebind;
mod range;
mod flat_map;
use ::rebind::Rebind;
use ::option::Optian;
use ::range::Range;
use ::fmap::FMap;
use ::flat_map::FlatMap;

pub trait Pure {
    fn pure_point<T>(obj: T) -> <Self as Rebind<T>>::Type
        where Self: Rebind<T>
        ;
}


impl<U> Pure for Optian<U> {
    fn pure_point<T>(obj: T) -> <Optian<U> as Rebind<T>>::Type
    {
        Optian::Some(obj)
    }
}

impl<S, U> FMap<U> for S
    where Self: FlatMap<U>,
          Self: Pure,
          Self::Element: Copy,
          Self::Type: Copy
{
    fn fmap<F>(&self, f: F) -> Self::Type
        where F: FnOnce(Self::Element) -> U
    {
        self.flat_map(|elem| Self::pure_point(f(elem)))
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
    println!("Hello, world!");
}
