use ::option::Optian;
use ::element::Element;

pub trait Traversable: Element + Sized
{
    fn next(&self) -> Optian<(Self, Self::Element)>;
}
