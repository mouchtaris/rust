use ::std::marker::PhantomData;

macro_rules! salias {
    ( $t:ty , $( $i:ident ),* ) => (
        $(
            #[derive(Debug)]
            struct $i($t);
        )*
    )
}
macro_rules! impl_from_string {
    ( $( $t:ident ),* ) => (
        $(
            impl $t
            {
                fn from_string(s: &str) -> $t { $t(s.to_string()) }
            }
        )*
    )
}
macro_rules! impl_from_usize {
    ( $( $t:ident ),* ) => (
        $(
            impl $t
            {
                fn from_u32(i: usize) -> $t { $t(i as u32) }
            }
        )*
    )
}

salias!(String, Email, Username);
impl_from_string!(Email, Username);
salias!([u8; 32], Password);

trait To<T> { fn to(&self) -> T; }

#[derive(Copy,Clone,Debug)]
struct Id<T>(u32, PhantomData<T>);
impl<T> To<Id<T>> for usize { fn to(&self) -> Id<T> { Id(*self as u32, PhantomData) } }

impl Password
{
    fn from_string(s: &str) -> [u8; 32]
    {
        let mut result: [u8; 32] = [0; 32];
        let mut i = 0;
        for b in s.bytes() {
            if i > 31 {
                break;
            }
            result[i] = b;
            i += 1;
        }
        while i < 31 {
            result[i] = 0;
            i += 1;
        }
        result
    }
}

#[derive(Debug)]
struct Account {
    email: Email,
    password: Password,
}

#[derive(Debug)]
struct User {
    account_id: Id<Account>,
    username: Username
}
impl User
{
    fn account<'a>(&self, repo: &'a Repo<Account>) -> &'a Account { repo.get(&self.account_id) }
}


struct Repo<T>
{
    items: Vec<T>,
}
impl<T> Repo<T>
{
    fn new() -> Repo<T> { Repo { items: Vec::new() } }

    fn persist(&mut self, elem: T) -> Id<T>
    {
        self.items.push(elem);
        self.items.len().to()
    }

    fn get(&self, id: &Id<T>) -> &T
    {
        &self.items[(id.0 - 1) as usize]
    }
}

fn main()
{
    let email = Email::from_string("mouchtaris@gmail.com");
    let password = Password(Password::from_string("Le bobs"));
    let account = Account { email, password, };

    let mut account_repo = Repo::<Account>::new();

    let username = Username::from_string("lolis");
    let user = User { username, account_id: account_repo.persist(account) };

    println!("{:?}", user);
    println!("{:?}", user.account(&account_repo));
    println!("Hello, world!");
}
