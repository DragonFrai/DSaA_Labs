use crate::n1search::MyBinaryTree;
use rand::{SeedableRng, Rng};
use crate::myhashmap::{ReHashMap, ChainHashMap};
use crate::chess::Chess;

mod n1search;
mod myhashmap;
mod chess;

fn main() {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut source = (0..20).map(|_| rng.gen_range(0..20)).collect::<Vec<_>>();

    let mut tree = MyBinaryTree::new();
    let mut remap = ReHashMap::new(source.len());
    let mut chain_map = ChainHashMap::new(source.len() / 2 + 1);

    for x in &source {
        tree.add(*x);
        remap.add(*x);
        chain_map.add(*x);
    }

    source.sort();
    println!("Исходный массив: {:?}", &source);

    let t = |x: bool| if x { "+" } else { "-" };
    println!("    Binary | Tree | Fib | Inter. | ReHash | Chains ");
    for i in 0..20 {
        let bin = n1search::binary_search(&source, &i).is_some();
        let tr = tree.has_value(&i);
        let fibs = n1search::fibonacci_search(&source, i).is_some();
        let inter = n1search::interpolation_search(&source, i).is_some();
        let rmap = remap.has_value(&i);
        let cmap = chain_map.has_value(&i);

        println!("{:02}: {:}        {:}      {:}     {:}        {:}        {:}", i, t(bin), t(tr), t(fibs), t(inter), t(rmap), t(cmap));
    }

    println!("\n");
    println!("Задача про ферзей");
    let mut chess = Chess::new();
    let table = chess.solve();
    chess.printBoard(&table);




}
