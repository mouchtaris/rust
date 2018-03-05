use ::rebind::Rebind;

pub trait FMap<U>
    where Self: Rebind<U>,
          Self::Element: Copy
{
    fn fmap<F>(&self, f: F) -> Self::Type
        where F: FnOnce(Self::Element) -> U
        ;
}
