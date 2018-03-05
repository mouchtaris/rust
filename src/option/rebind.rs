use ::rebind::*;
use super::Optian;

impl<T, U> Rebind<U> for Optian<T> {
    type Element = T;
    type Type = Optian<U>;
}

