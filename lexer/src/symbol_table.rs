use std::collections::LinkedList;

pub type SymbolIdx = (usize, usize);

#[derive(Debug, Clone)]
pub struct SymbolTable<'a> {
    hash_table: Vec<LinkedList<&'a str>>,
}

fn bad_hash(s: &str) -> u64 {
    static A: u64 = 10000007u64;
    static B: u64 = 99999949u64;
    static MOD: u64 = 132015214213u64;

    s.chars()
        .fold((1u64, 0u64), |(a_pow, _hash), item| {
            (a_pow * A % MOD, (a_pow * (item as u64) + B) % MOD)
        })
        .1
}

impl<'a> SymbolTable<'a> {
    pub fn new(buckets: usize) -> SymbolTable<'a> {
        SymbolTable {
            hash_table: vec![LinkedList::new(); buckets],
        }
    }

    pub fn add(&mut self, symbol: &'a str) -> SymbolIdx {
        let hsh = bad_hash(symbol) as usize;
        let table_idx = hsh % self.hash_table.len() as usize;

        let list = &mut self.hash_table[table_idx as usize];

        match list.iter().position(|&s| s == symbol) {
            Some(i) => (table_idx, i),
            None => {
                list.push_back(symbol);
                (table_idx, list.len() - 1)
            }
        }
    }
}

impl<'a> std::fmt::Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (table_idx, list) in self.hash_table.iter().enumerate() {
            for (list_idx, symbol) in list.iter().enumerate() {
                writeln!(f, "{}: {:?}", symbol, (table_idx, list_idx))?;
            }
        }
        Ok(())
    }
}
