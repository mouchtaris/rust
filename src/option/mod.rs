mod rebind;
mod flat_map;
mod pure_point;

#[derive(Debug, Copy, Clone)]
pub enum Optian<T> {
    Some(T),
    None
}
