# Overview
This chapter covered the following concepts
- Printing and formatting text
- [[#Types]] - Working with strings (and types)
- Using `for`, `loop` and `if`.
- [[#Arrays and Vectors]]
- [[#Structs]]
- [[#Enumerations]]
- [[#Pattern matching with enum]]

The code for this chapter is in [the treehouse directory](/treehouse)

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


## Enumerations
We can define a type which can only be equal to a set of predefined values with `enum`:

```rust
#[derive(Debug)]
enum VisitorAction {
	Accept,
	AcceptWithNote { note: String },
	Refuse,
	Probation,
}
```

Variables from an enumeration can be assigned with `let visitor_action = VisitorAction::Accept;`. Actions with additional data can be assigned with
```rust
let visitor_action = VisitorAction::AcceptWithNote{note: "This is a note for the action".to_string()};
```

### Pattern matching with `enum`

**Pattern matching** serves two basic purposes:
- Checks to see if a condition is true and runs the associated code
- Extract fields from complicated types

```rust
match visitor_action {
	VisitorAction::Accept => println!("Welcome!"),
	VisitorAction::Probation => {
		do_something_more_involved();
	}
	VisitorAction::AcceptWithNote { note } => {
		println!("{}", note);
	}
	_ => println!("No entry!");
}
```

The syntax for `VisitorAction::AcceptWithNote { note } => {printl!("{}", note);}` allows us to capture the `note` data which is associated with the action in the `enum` defintiion - this is called _destructuring_. 
The `_` case is used as a 'all other cases' (or `else`) statement.

## Types
- `String` is stored as a Vector of bytes (`Vec<u8>`) but is guranteed to be a valid UTF-8 sequence. It is heap allocated, growable (mutable size) and not null terminated.
- `&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence and can be used as a _view_ into a `String`. So `&str` is a static version of the _mutable_ `String`.
- `i8` is an 8-bit signed integer (with values from -128 to 127)

You construct a string with
```rust
String::from("this is a string")
```