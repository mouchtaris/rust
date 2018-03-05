mod rebind;
mod fmap;
mod flat_map;

#[derive(Debug, Copy, Clone)]
pub enum Optian<T> {
    Some(T),
    None
}

