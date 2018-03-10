#[macro_use]
mod pdb;
mod make;
mod id;
use ::make::{ Make };
use ::id::Id;

#[derive(Debug)]
struct Bob;

trait Repo<T>
{
    fn persist(&mut self, t: T) -> Id<T>;
    fn retrieve<'s>(&'s self, id: &Id<T>) -> &'s T;
}

struct FUPDRepo<T>
{
    items: Vec<T>,
}
impl<T> Repo<T> for FUPDRepo<T>
{
    fn persist(&mut self, t: T) -> Id<T>
    {
        self.items.push(t);
        Make.make::<Id<T>>(self.items.len() as u32)
    }

    fn retrieve<'s>(&'s self, id: &Id<T>) -> &'s T
    {
        &self.items[(id.0  - 1) as usize]
    }
}
impl<T> FUPDRepo<T>
{
    fn new() -> FUPDRepo<T> { FUPDRepo { items: vec![] } }
}

struct BobContext
{
    repo: FUPDRepo<Bob>,
}
struct Context
{
    bob: BobContext,
}
impl Context
{
    fn new() -> Context
    {
        Context {
            bob: BobContext {
                repo: FUPDRepo::new(),
            },
        }
    }
}


fn main()
{
    let mut ctx = Context::new();
    let id = ctx.bob.repo.persist(Bob);
    pdb!(id);
    let bob = ctx.bob.repo.retrieve(&id);
    pdb!(bob);
}
