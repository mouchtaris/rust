mod can_make_from_u32;
use ::std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T>(
    u32,
    PhantomData<T>,
);
