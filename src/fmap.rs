use rebind::*;

pub trait FMap<U>
    where Self: Rebind<U>,
          Self::Element: Copy
{
    fn fmap(
        &self,
        f: fn(Self::Element) -> U
    ) -> Self::Type;
}
