### hashmap-rs

Building a simple HashMap in Rust. Follows [live stream](https://www.youtube.com/watch?v=k6xR2kf9hlA) done by Jon Gjengset. 


### Technical notes:
- HashMap requires another object type apart from key and value. That type should implement the trait BuildHasher. Anything that implements BuildHasher has to implement a function `build_hasher` that has to return the type that implements trait `Hasher`. 

### Notes:
- Building a library through example
	-  This is very much like doing TDD. Think how user of the library will use it and develop from there. This will force you to think hard about the features and non-features and make the scope very concrete.
	-	We can directly take the [example](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#examples) that rust docs list for std::collections::HashMap!
- What is `extern crate` doing?
	- See [this answer](https://stackoverflow.com/a/54378840/12764266) and [official documentation](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html).
- Jon: "You should have trait bounds on the methods rather on the data structures themselves." 
	- Standard implementation of HashMap only applies trait bounds on the implementation and not on the type. 
	- I don't see this being good or bad, just a matter of preference. 
- Why have `impl<T> SomeType<T>` and not `impl SomeType<T>`?
	- "...we have to declare `T` just after `impl` so we can use it to specify that we’re implementing methods on the type `Point<T>`. By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type." from [rust book](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-method-definitions)
- Need to pick a default state for the buckets. The general way would be to pick the power of two. But we should not do that in rust, creating an empty new vector for a bucket is free and better.
- A tradeoff between having low number of buckets and very high number of buckets.
	- Low number of buckets would mean we would not have to perform the costly resize operation.
	- High number of buckets would mean large memory required.
- Why `insert` method returns `Option<V>`?
	- insert returns something if we are performing an overwrite operation.
- What is difference between `&` and `ref`?
	- In destructuring subpatterns the `&` operator can't be applied to the value's fields. `ref`s objective is exclusively to make the matched binding a reference, instead of potentially copying or moving what was matched. [source](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)
- 