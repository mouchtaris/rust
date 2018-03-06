mod fmap;
mod option;
mod rebind;
mod range;
mod flat_map;
mod pure_point;
use ::rebind::Rebind;
use ::option::Optian;
use ::range::Range;
use ::fmap::FMap;
use ::flat_map::FlatMap;
use ::pure_point::PurePoint;


pub trait Iter
    where Self: Sized
{
    type Element;
    fn next(&self) -> Optian<(Self, Self::Element)>;
}


pub struct ABC
{
    a: i32,
    b: i32,
    c: i32
}
mod ABCIter {
    use ::ABC;
    use ::option::Optian;
    use ::fmap::FMap;
    #[derive(Clone,Copy)]
    enum State { a, b, c, e }
    pub struct Iter<'a> { src: &'a ABC, state: State }
    impl<'a> ::std::clone::Clone for Iter<'a>
    {
        fn clone(&self) -> Self
        {
            Iter { .. *self }
        }
    }
    impl<'a> ::std::marker::Copy for Iter<'a>
    {
    }
    impl <'a> ::Iter for Iter<'a>
    {
        type Element = i32;
        fn next(&self) -> Optian<(Iter<'a>, i32)>
        {
            let (nextState, maybeValue) = match self.state {
                State::a => (State::b, Optian::Some(self.src.a)),
                State::b => (State::c, Optian::Some(self.src.b)),
                State::c => (State::e, Optian::Some(self.src.c)),
                State::e => (State::e, Optian::None)
            };
            maybeValue.fmap(|v| {
                let nextSelf = Iter { src: self.src, state: nextState };
                (nextSelf, v)
            })
        }
    }
    pub fn create(src: &ABC) -> Iter { Iter { src, state: State::a } }
}

impl ABC
{
    fn iter<'a>(&'a self) -> ABCIter::Iter<'a> { ABCIter::create(self) }
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
