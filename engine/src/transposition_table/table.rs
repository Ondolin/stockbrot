use std::sync::RwLock;
use crate::transposition_table::entry::Entry;

use rayon::prelude::*;

const BUCKET_SIZE: usize = 3;
const TABLE_SIZE: usize = 3_000_000;

type Bucket = [RwLock<Entry>; BUCKET_SIZE];

trait BucketFuncs {
    fn get_by_hash(& self, hash_value: u64) -> &RwLock<Entry>;
}

impl BucketFuncs for [RwLock<Entry>; BUCKET_SIZE] {
    fn get_by_hash(&self, hash_value: u64) -> &RwLock<Entry> {

        let mut found: i8 = -1;
        let mut free: i8 = -1;

        let mut max_age: i8 = -1;
        let mut max_age_index: i8 = -1;

        for (index, entry) in self.iter().enumerate() {
            match *entry.read().unwrap() {
                Entry::Contains { hash, age, .. } => {
                    if hash_value == hash { found = index as i8; break; }
                    if max_age < age as i8 { max_age_index = index as i8; max_age = age as i8 }
                },
                Entry::Empty => { free = index as i8 }
            }
        }

        if found >= 0 { return &self[found as usize] }
        if free >= 0 { return &self[free as usize] }

        if max_age_index < 0 {
            &self[0]
        } else {
            &self[max_age_index as usize]
        }


    }
}

pub struct TranspositionTable {
    buckets: Vec<Bucket>
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {

        let mut buckets = Vec::with_capacity(TABLE_SIZE);

        for _ in 0..TABLE_SIZE {
            let entry = Entry::Empty;
            let bucket: Bucket = core::array::from_fn(|_| RwLock::new(entry.clone()));

            buckets.push(bucket);
        }

        TranspositionTable {
            buckets
        }
    }

    pub fn get(&self, hash: u64) -> &RwLock<Entry> {
        let bucket: &Bucket = &self.buckets[hash as usize % TABLE_SIZE];

        let entry = bucket.get_by_hash(hash);

        entry
    }

    pub fn age_table(&self) {
        self.buckets.par_iter().for_each(|bucket| {
            for entry in bucket.iter() {
                let mut entry = entry.write().unwrap();

                match *entry {
                    Entry::Contains { hash, depth, score, age, node_type } => { *entry = Entry::Contains { hash, depth, score, age: age + 1, node_type } }
                    Entry::Empty => {}
                }

            }
        });
    }
}