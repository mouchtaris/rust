pub trait CanMake<T>
{
    type Args;
    fn then_make_it(&self, args: Self::Args) -> T;
}

