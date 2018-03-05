use super::Optian;
use ::rebind::Rebind;
use ::pure_point::PurePoint;

impl<U> PurePoint for Optian<U> {
    fn pure_point<T>(obj: T) -> <Optian<U> as Rebind<T>>::Type
    {
        Optian::Some(obj)
    }
}
