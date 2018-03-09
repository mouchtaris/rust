use super::Id;
use ::std::marker::PhantomData;
use ::make::{ Make, CanMake };

impl<T> CanMake<Id<T>> for Make
{
    type Args = u32;
    fn then_make_it(&self, id: u32) -> Id<T>
        { Id(id, PhantomData) }
}

