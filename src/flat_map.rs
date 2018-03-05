use ::rebind::*;

pub trait FlatMap<U>
    where Self: Rebind<U>,
          Self::Element: Copy,
          Self::Type: Copy
{
    fn flat_map(&self, f: fn(Self::Element) -> Self::Type) -> Self::Type;
}
