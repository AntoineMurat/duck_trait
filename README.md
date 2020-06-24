# Duck typing for Rust

```duck_trait``` is a crate aiming at making duck typing very easy and fast.

⚠️ I am not pretending that this is a good pattern nor that it should be used
in real-life scenarios.

⚠️ It hasn't been properly tested yet. It will not work (with some high probability).

## How to import

Add the crate dependency to your ```Cargo.toml```.

```toml
[dependencies]
duck_trait = { git = "https://github.com/AntoineMurat/duck_trait" }
```

## How to use

Let's say that you have two structs ```Dog``` and ```Duck``` sharing some
fields and methods (that have the *same* name and types).

```rust
struct Dog { weight: u32 }

impl Dog {
    fn make_noise(&self) { println!("Woof"); }
    fn make_static_noise() { println!("Static woof"); }
}

struct Duck { weight: u32 }

impl Duck {
    fn make_noise(&self) { println!("Quack"); }
    fn make_static_noise() { println!("Static quack"); }
}
```

You might want to create a function/struct/whatever that can take either a 
```Dog``` or a ```Duck```. Doing so would require you to define a common trait
that you should then implement for both structs.

Using ```duck_trait```, this boilerplate is done within a macro.

```rust
use duck_trait::duck_trait;

duck_trait!(Animal, 
    [Dog, Duck],
    [weight: u32;],
    [fn make_static_noise(); fn make_noise(&self);]
);

fn measure_weight_twice<T: Animal>(animal: &mut T) -> u32 {
    *animal.weight_mut() *= 2;
    animal.make_noise();
    T::make_static_noise();
    *animal.weight()
}

fn main() {
    let mut duck = Duck { weight: 4 };
    let mut dog = Dog { weight: 6 };

    assert_eq!(measure_weight_twice(&mut duck), 8);
    assert_eq!(measure_weight_twice(&mut dog), 12);
}
```

The syntax is:

```
duck_trait!(duck_tait_name, 
    [struct1, struct2, struct3, etc.],
    [field1: field1_type; field2: field2_type; etc.],
    [function1; function2; etc.]
);
```

You can then be generic over ```duck_tait_name```.

Fields can be accessed using the ```field()``` method and can be modified using
the ```field_mut()``` method. Data can be moved out of struct using
```std::mem::replace```.

If you don't plan on using any "static method", you can use the duck trait to
manipulate trait objects.

```rust
use duck_trait::duck_trait;

duck_trait!(Animal, 
    [Dog, Duck],
    [weight: u32;],
    [fn make_noise(&self);]
);

fn measure_weight_twice(animal: &mut dyn Animal) -> u32 {
    *animal.weight_mut() *= 2;
    animal.make_noise();
    *animal.weight()
}
```

## What doesn't work

Function parameters should not involve pattern matching as every function
parameter is expected to follow the pattern $arg:ident: $type:ty.