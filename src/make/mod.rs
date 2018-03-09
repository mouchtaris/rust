mod can_make;
pub use self::can_make::CanMake;

pub struct Make;

impl Make
{
    pub fn make<T>(
        &self,
        args: <Self as CanMake<T>>::Args,
        )
    -> T
    where Self: CanMake<T>
    {
        self.then_make_it(args)
    }
}

