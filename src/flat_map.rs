use ::rebind::Rebind;

pub trait FlatMap<U>
    where Self: Rebind<U>,
          Self::Element: Copy,
          Self::Type: Copy
{
    fn flat_map<F>(&self, f: F) -> Self::Type
        where F: FnOnce(Self::Element) -> Self::Type
        ;
}
