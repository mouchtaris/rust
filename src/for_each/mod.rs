mod from_traversable;
use ::element::Element;

pub trait ForEach: Element
{
    fn for_each<F>(&self, f: F)
        where F: Fn(Self::Element)
        ;
}

