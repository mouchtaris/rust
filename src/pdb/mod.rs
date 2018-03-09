#[macro_export]

macro_rules! pdb {
    ( $id:ident ) => ( println!("{:?}", $id) )
}
