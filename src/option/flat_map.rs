use super::Optian;
use ::flat_map::*;

impl<T, U> FlatMap<U> for Optian<T>
    where T: Copy,
          U: Copy
{
    fn flat_map(&self, f: fn(T) -> Optian<U>) -> Optian<U> {
        match self {
            &Optian::Some(t) => f(t),
            &Optian::None => Optian::None
        }
    }
}

