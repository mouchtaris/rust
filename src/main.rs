trait Element { type Element; }
trait Rebind<U> { type Rebound; }
trait FMap<U>: Element + Rebind<U> { fn fmap<F: Fn(&Self::Element) -> U>(&self, f: F) -> Self::Rebound; }
trait FlatMap<U>: Element + Rebind<U> { fn flat_map<F: Fn(&Self::Element) -> Self::Rebound>(&self, f: F) -> Self::Rebound; }
trait PurePoint { fn pure_point<T>(t: T) -> Self::Rebound where Self: Rebind<T>; }
trait Traversing<T>: Sized 
{
    type Next: Traversing<T>;
    fn fnext(&self) -> Option<(Self::Next, &T)>;
    fn feach<F: Fn(&T)>(&self, f: F)
    {
        match self.fnext() {
            Option::Some((ref next, ref t)) => {
                f(t);
                next.feach(f);
            },
            Option::None =>
                (),
        }
    }
}


macro_rules! impl_fmap_from_flat_map_and_pure_point {
    ( $t:ident ) => (
        impl<T, U> FMap<U> for $t<T>
        {
            fn fmap<F: Fn(&T) -> U>(&self, f: F) -> $t<U>
            {
                self.flat_map(|e| Self::pure_point(f(e)))
            }
        }
    )
}


//
// Option Implementations
// 
impl<T> Element for Option<T> { type Element = T; }
impl<T, U> Rebind<U> for Option<T> { type Rebound = Option<U>; }
impl<T, U> FlatMap<U> for Option<T>
{
    fn flat_map<F: Fn(&T) -> Option<U>>(&self, f: F) -> Option<U>
    {
        match self {
            &Option::Some(ref t) => f(t),
            &Option::None => Option::None,
        }
    }
}
impl<T> PurePoint for Option<T>
{
    fn pure_point<U>(u: U) -> <Option<T> as Rebind<U>>::Rebound
    {
        Option::Some(u)
    }
}
impl_fmap_from_flat_map_and_pure_point!(Option);
impl<T> Traversing<T> for Option<T>
{
    type Next = Self;
    fn fnext(&self) -> Option<(Option<T>, &T)>
    {
        match self {
            &Option::Some(ref t) => Option::Some((Option::None, t)),
            &Option::None => Option::None,
        }
    }
}


//
// New Bobs
//
#[derive(Copy, Clone, Debug)]
struct Cons<A, B> { head: A, tail: B }
#[derive(Copy, Clone, Debug)]
struct Nil;
fn cons<A, B>(head: A, tail: B) -> Cons<A, B> { Cons { head, tail } }

fn main()
{
    let pln = |x: &i32| println!("{:?}", x);
    let opt = Option::Some(12);
    opt.feach(pln);
    let li = cons(12, cons("hello", cons(true, cons(23.354, Nil))));

    println!("{:?}", Option::Some(12).fnext().fmap(|x| x.1.to_string()));
    println!("{:?}", li);
    println!("Hello, world!");
}
