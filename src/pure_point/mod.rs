use ::rebind::Rebind;

pub trait PurePoint {
    fn pure_point<T>(obj: T) -> <Self as Rebind<T>>::Type
        where Self: Rebind<T>
        ;
}
