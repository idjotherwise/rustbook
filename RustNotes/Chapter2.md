## Structs
Structs hold data and can be defined as
```rust
struct Visitor {
	name: String,
	age: Int,
	greeting: String,
}
```

the trailing comma on the last entry is ignored by the compiler, so it is optional.

To implement methods associated with the struct, you do something like

```rust
impl Visitor {
	fn new(name: &str, age: &int, greeting: &str) -> Self {
		Self {
			name: name.to_lowercase(),
			age: age,
			greeting: greeting.to_string(),
		}
	}
	fn greet_visitor(&self) {
		println!("{}", self.greeting);
	}
}
```

## Arrays and Vectors
**Arrays** have a _fixed size_, while **Vectors** (`Vec`) are dynamically resizable. Elements can be added with `push()`.

Vectors can be created in a similar way to arrays, with the help of the `vec!` macro: 

```rust
# array, immutable size:
let visitor_list: [&str, 2] = [
"visitor0",
"visitor1",
];

# vec! macro, mutable size
let mut visitor_list = vec![
"visitor0",
"visitor1",
];

# or without the vec! macro:
let mut visitor_list = Vec::new();
visitor_list.push(
	"visitor0"
);
...


```

Vectors are generally declared as `Vec{T}` for some type `T`, same as in `julia`.

## Debug printing
To print an instance of a (custom) struct, you need to add the `#[derive(Debug)]` to the struct definition like so:
```rust
#[derive(Debug)]
struct Visitor {
	name: String,
	greeting: String,
}
```
then you can print all the information about the instance with:

```rust
println!("{:?}", Visitor::new("ifan", "hello there"))
```
or `"{:#?}"` for pretty printing.


## Loops and breaks
Syntax for running a loop until you break it is:
```rust
loop {
printl!("This is a loop!")
... // loop is evaluated repeatedly until break is called
break;
}
// execution resumes here after break
```


## Iterators and options
`array.iter()` will create an iterator from the items of `array` and then calling `.find(|item| condition)` will return `item`s that satisfy `condition` . If no items satsify `condition` then it will return `None`. To deal with this you can use match to check for `something` or `nothing`:

```rust
match visitor_list {
	Some(visitor) => visitor.greet_visitor(),
	None => {
		// deal with missing match
	}
}
```
