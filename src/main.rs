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

}

fn main()
{
    let wot = Make.make::<Id<Bob>>(218);
    pdb!(wot);
}
