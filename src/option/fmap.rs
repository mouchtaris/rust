use ::fmap::*;
use super::Optian;

impl<T, U> FMap<U> for Optian<T>
    where T: Copy
{
    fn fmap(&self, f: fn(T) -> U) -> Optian<U> {
        match self {
            &Optian::Some(t)=> Optian::Some(f(t))   ,
            &Optian::None   => Optian::None
        }
    }
}

