
use std::collections::HashSet;
use std::{collections::HashMap, hash::RandomState};
use std::hash::{BuildHasher, Hash};

use pipe::{Pipeable};

pub trait IntoMap{
	type Key;
	type Value;
	fn collect_map(self) -> HashMap<Self::Key, Self::Value>;
	fn collect_map_with_hasher<S: BuildHasher>(self, hasher: S) -> HashMap<Self::Key, Self::Value, S>;
}

impl<T, Key, Value> IntoMap for T 
	where 
		T: Iterator<Item=(Key, Value)>,
		Key: Hash + Eq
{
	type Key =Key;
	type Value = Value;

	fn collect_map(self) -> HashMap<Key, Value, RandomState>{
		self.collect()
	}

	fn collect_map_with_hasher<S: BuildHasher>(self, hash_builder: S) -> HashMap<Key, Value, S>{
		HashMap::with_hasher(hash_builder)
		.pipe_apply(|x| x.extend(self))
	}
}

pub trait IntoSet {
	type Value;
	#[inline(always)]
	fn collect_set(self) -> HashSet<Self::Value>;

	#[inline(always)]
	fn collect_set_with_hasher<S: BuildHasher>(self, hash_builder: S) -> HashSet<Self::Value, S>;
}

impl<T, Value> IntoSet for T where T:Iterator<Item=Value>, Value: Eq + Hash{
	type Value = Value;

	#[inline(always)]
	fn collect_set(self) -> HashSet<Self::Value> {
		self.collect()
	}

	#[inline(always)]
	fn collect_set_with_hasher<S: BuildHasher>(self, hash_builder: S) -> HashSet<Self::Value, S> {
		HashSet::with_hasher(hash_builder)
		.pipe_apply(|x| x.extend(self))	
	}
}

// impl<T, Key, Value> CollectMap<Key, Value> for T 
// 	where HashMap<Key, Value>: FromIterator 
// 	{

// 	} 
fn test(){
	let a  = [1,2,3];
	a.into_iter().collect_set();	
}