# hashmap-rs

Building a simple HashMap in Rust. Follows [live stream](https://www.youtube.com/watch?v=k6xR2kf9hlA) done by Jon Gjengset. 


## Implementation:

The HashMap currently supports a subset of the standard library HashMap's [implementations](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#implementations):

#### `HashMap`
- Methods: 
	- `new`
	- `bucket`
	- `entry`
	- `insert`
	- `get`
	- `contains_key`
	- `remove`
	- `resize`
	- `len`
	- `is_empty`
- Traits:
	- `Index`
	- `From`
	- `IntoIterator`
	- `FromIterator`
#### `Entry`
- Methods:
	- `or_insert`
	- `or_insert_with`
	- `or_default`

### Technical Notes:
- Map structure:
	```
	BUCKETS
	-----     -------
	| 0 | ->  | k,v |
	-----     -------------
	| 1 | ->  | k,v | k,v |
	-----     -------------
	| 2 | ->  
	-----     -------
	| 3 | ->  | k,v |
	-----     -------------------------
	| 4 | ->  | k,v | k,v | k,v | k,v |
	-----     -------------------------
	 .
	 .
	 .
	

	 k,v: key value pair
	```
- HashMap created by `new` method should not allocate any memory. It will be of capacity 0.  
- Map will be resized when it is either empty or 3/4 full. On every resize we double the capacity.
- Key must implement `Hash + Eq`. `Hash` required so that key can be hashed and mapped to a bucket number. `Eq` is required so that it can be checked if map contains an (key, value) entry or not.
- If `insert` method is called and the key is already present, use `std::mem::replace`. With `replace`, owning the values or expecting them to be `Clone`able is [not required](https://doc.rust-lang.org/std/mem/fn.replace.html#examples).	
- `get` operation should be generic over the underlying key data. If key is of type `String`, we should be able to get with type `&str`. If the owned key type is K, then `get` [should be generic over Q](https://doc.rust-lang.org/std/borrow/trait.Borrow.html#examples) such that `K: Borrow <Q>` and `Q: [whatever K is required to have]`. 
- Use `Vec::swap_remove` to efficiently remove an entry in `remove` method.
-  `IntoIterator::into_iter` returns a type that implements `Iterator::next`. IntoIterator implementation for map exists for a ref to map `... IntoIterator for &'a HashMap<K, V>` and owned map `... IntoIterator for HashMap<K, V>`.
- To manage lifetimes while implementing `entry` method, determine the `Entry` type first (whether `Entry::Occupied` or `Entry::Vacant`) and then return relevant type. (This way `unsafe` usage can be avoided. [Implementation in livestream uses `unsafe`](https://youtu.be/k6xR2kf9hlA?t=7214))


## Notes:
- Idea of building a library through example.
	- This is very much like doing TDD. Think how user of the library will use it and develop from there. This will force you to think hard about the features and non-features and make the scope very concrete.
	- We can directly take the [example](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#examples) that rust docs list for std::collections::HashMap!
- Jon: "You should have trait bounds on the methods rather on the data structures themselves." 
	- Standard implementation of HashMap only applies trait bounds on the implementation and not on the type. 	
- What is difference between `&` and `ref`?
	- In destructuring subpatterns the `&` operator can't be applied to the value's fields. `ref`s objective is exclusively to make the matched binding a reference, instead of potentially copying or moving what was matched. See [source](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns).
- `unreachable!()` -> is a neat macro that will just throw the error when it is invoked.
- Generics + Lifetimes?
	- Rust book [ch 10](https://doc.rust-lang.org/stable/book/ch10-00-generics.html).























