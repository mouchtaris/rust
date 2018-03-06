use super::ForEach;
use ::traversable::Traversable;
use ::option::Optian;

impl <S: Traversable> ForEach for S
{
    fn for_each<F>(&self, f: F)
        where F: Fn(Self::Element)
    {
        match self.next() {
            Optian::Some((slf, elm)) => {
                f(elm);
                slf.for_each(f);
            },
            Optian::None =>
                ()
        }
    }
}


