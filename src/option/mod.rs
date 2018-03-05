mod rebind;
mod fmap;

#[derive(Debug)]
pub enum Optian<T> {
    Some(T),
    None
}

