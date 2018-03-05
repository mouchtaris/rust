use super::FMap;
use ::flat_map::FlatMap;
use ::pure_point::PurePoint;

impl<S, U> FMap<U> for S
    where Self: FlatMap<U>,
          Self: PurePoint,
          Self::Element: Copy,
          Self::Type: Copy
{
    fn fmap<F>(&self, f: F) -> Self::Type
        where F: FnOnce(Self::Element) -> U
    {
        self.flat_map(|elem| Self::pure_point(f(elem)))
    }
}
