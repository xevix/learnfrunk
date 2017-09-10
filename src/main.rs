trait Folder: Iterator {
    fn foldl1<F>(mut self, f: F) -> Option<Self::Item>
        where
            F: FnMut(Self::Item, Self::Item) -> Self::Item;
}

impl<I> Folder for I where I: Iterator {
    fn foldl1<F>(mut self, f: F) -> Option<Self::Item>
        where
            F: FnMut(Self::Item, Self::Item) -> Self::Item
    {
        if let Some(init) = self.next() {
            let mut accum = init;
            for x in self {
                accum = f(accum, x);
            }
            Some(accum)
        } else {
            None
        }

    }
}

fn combine_all_optionf<T>(xs: &Vec<T>) -> Option<T>
    where
        T: Semigroup + Clone
{
    xs.iter().foldl1(|acc, next| acc.combine(next))
}


extern crate frunk;
use frunk::semigroup::*;

fn main() {
    let vec = vec![1, 2, 3];
    println!("Vec: {:?}", vec);
    let res = combine_all_optionf(&vec);
    match res {
        Some(num) => println!("Result is: {}", num),
        None => println!("Empty vec")
    }
    println!("Hello, world!");
}
