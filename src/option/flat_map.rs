use super::Optian;
use ::flat_map::FlatMap;

impl<T, U> FlatMap<U> for Optian<T>
    where T: Copy,
          U: Copy
{
    fn flat_map<F>(&self, f: F) -> Optian<U>
        where F: FnOnce(T) -> Optian<U>
    {
        match self {
            &Optian::Some(t) => f(t),
            &Optian::None => Optian::None
        }
    }
}

