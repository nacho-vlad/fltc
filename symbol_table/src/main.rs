use std::collections::LinkedList;

struct SymbolTable<'a> {
    hash_table: Vec<LinkedList<(&'a str, u32)>>,
}

fn bad_hash(s: &str) -> u64 {
    static A: u64 = 10000007u64;
    static B: u64 = 99999949u64;
    static MOD: u64 = 132015214213u64;

    s.chars().fold((1u64, 0u64), |(a_pow, hash), item| {
        (a_pow * A % MOD, (a_pow * (item as u64) + B) % MOD)
    }).1
}

impl<'a> SymbolTable<'a> {
    
    fn new(buckets: usize) -> SymbolTable<'a> {
        SymbolTable {
            hash_table: vec![LinkedList::new(); buckets]
        }
    }
    

    fn add(&mut self, symbol: &'a str, idx: u32) -> u32 {
        let hsh = bad_hash(symbol);
        let table_idx = hsh % self.hash_table.len() as u64;
        
        let list = &mut self.hash_table[table_idx as usize];
        
        match list.iter().find(
            |&&(s, i)| {
                s == symbol 
            }) {
            Some(&(_, i)) => i,
            None => {
                list.push_back((symbol, idx));
                idx
            }
        }
    }

}


fn main() {
    let mut st = SymbolTable::new(100);
    println!("{}", st.add("aa", 2));
    println!("{}", st.add("vb", 4));
    println!("{}", st.add("aa", 3));

    println!("Hello, world!");

}
