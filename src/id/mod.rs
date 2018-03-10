mod can_make_from_u32;
use ::std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T>(
    pub u32,
    PhantomData<T>,
);
