#![feature(trace_macros)]

use proc_macros::*;

trace_macros!(true);

#[macro_export]
macro_rules! duck_trait {
    ($trait:ident, [$($duck_struct:ident),*]) => {
        duck_trait!($trait, [$($duck_struct),*], []);
    };
    ($trait:ident, [$($duck_struct:ident),*], [$($field:ident: $type:ty;)*]) => {
        duck_trait!($trait, [$($duck_struct),*], [$($field: $type;)*], []);
    };
    ($trait:ident, [$($duck_struct:ident),*], [$($field:ident: $type:ty;)*], [$($fns_tokens:tt)*]) => {
        define_trait!($trait, [$($field: $type;)*], [$($fns_tokens)*]);
        impl_trait!($trait, [$($duck_struct),*], [$($field: $type;)*], [$($fns_tokens)*]);
    };
}

macro_rules! define_trait {
    ($trait:ident, [$($field:ident: $type:ty;)*], [$($fns_tokens:tt)*]) => {
        trait $trait {
            define_fields!($($field: $type;)*);
            $($fns_tokens)*
        }
    };
}

macro_rules! define_fields {
    () => {};
    ($field:ident: $type:ty; $($fields:ident: $types:ty;)*) => {
        fn $field(&self) -> &$type;
        paste::item! {
            fn [<$field _mut>](&mut self) -> &mut $type;
        }
        define_fields!($($fields: $types;)*);
    };
}

macro_rules! impl_trait {
    ($trait:ident, [], [$($field:ident: $type:ty;)*], [$($fns_tokens:tt)*]) => {};
    ($trait:ident, [$duck_struct:ident], [$($field:ident: $type:ty;)*], [$($fns_tokens:tt)*]) => {
        impl $trait for $duck_struct {
            impl_fields!($($field: $type;)*);
            impl_fns!($($fns_tokens)*);
        }
    };
    ($trait:ident, [$duck_struct:ident, $($other_structs:ident),*], [$($field:ident: $type:ty;)*], [$($fns_tokens:tt)*]) => {
        impl_trait!($trait, [$duck_struct], [$($field: $type;)*], [$($fns_tokens)*]);
        impl_trait!($trait, [$($other_structs),*], [$($field: $type;)*], [$($fns_tokens)*]);
    };
}

macro_rules! impl_fns {
    () => {};
    ($($fns_tokens:tt)*) => {
        before_semicolon!(impl_fn $($fns_tokens)*);
        after_semicolon!(impl_fns $($fns_tokens)*);
    };
}

macro_rules! impl_fn {
    (fn $fn_name:ident(self) $($return_tokens:tt)*) => {
        fn $fn_name(self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (fn $fn_name:ident(self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        fn $fn_name(self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.$fn_name($($arg_name),*)
        }
    };
    (fn $fn_name:ident(&self) $($return_tokens:tt)*) => {
        fn $fn_name(&self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (fn $fn_name:ident(&self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        fn $fn_name(&self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.$fn_name($($arg_name),*)
        }
    };
    (fn $fn_name:ident(&mut self) $($return_tokens:tt)*) => {
        fn $fn_name(&mut self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (fn $fn_name:ident(&mut self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        fn $fn_name(&mut self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.fn_name($($arg_name),*)
        }
    };
    (fn $fn_name:ident($($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        fn $fn_name($($arg_name: $($arg_type)*)*) $($return_tokens)* {
            Self::$fn_name($($arg_name),*)
        }
    };
    (async fn $fn_name:ident(self) $($return_tokens:tt)*) => {
        async fn $fn_name(self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (async fn $fn_name:ident(self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        async fn $fn_name(self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.$fn_name($($arg_name),*)
        }
    };
    (async fn $fn_name:ident(&self) $($return_tokens:tt)*) => {
        async fn $fn_name(&self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (async fn $fn_name:ident(&self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        async fn $fn_name(&self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.$fn_name($($arg_name),*)
        }
    };
    (async fn $fn_name:ident(&mut self) $($return_tokens:tt)*) => {
        fn $fn_name(&mut self) $($return_tokens)* {
            self.$fn_name()
        }
    };
    (async fn $fn_name:ident(&mut self, $($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        fn $fn_name(&mut self, $($arg_name: $($arg_type)*)*) $($return_tokens)* {
            self.fn_name($($arg_name),*)
        }
    };
    (async fn $fn_name:ident($($arg_name:tt : $($arg_type:tt)*),*) $($return_tokens:tt)*) => {
        async fn $fn_name($($arg_name: $($arg_type)*)*) $($return_tokens)* {
            Self::$fn_name($($arg_name),*)
        }
    };
}

macro_rules! impl_fields {
    () => {};
    ($field:ident: $type:ty; $($fields:ident: $types:ty;)*) => {
        fn $field(&self) -> &$type { &self.$field }
        paste::item! {
            fn [<$field _mut>](&mut self) -> &mut $type { &mut self.$field }
        }
        impl_fields!($($fields: $types;)*);
    };
}


#[test]
fn generic_trait() {
    struct Dog {
        weight: u32,
        _name: String,
    }

    impl Dog {
        fn be_noisy(&self) {
            println!("Wouaf");
        }
        fn static_noise() {
            println!("Static wouaf");
        }
    }

    struct Duck {
        weight: u32
    }

    impl Duck {
        fn be_noisy(&self) {
            println!("Quack");
        }
        fn static_noise() {
            println!("Static quack");
        }
    }

    let mut duck = Duck { weight: 4 };
    let mut dog = Dog { weight: 2, _name: "".to_string() };

    duck_trait!(Animal, [Dog, Duck], [weight: u32;], [fn static_noise(); fn be_noisy(&self);]);

    fn measure_weight_twice<T: Animal>(animal: &mut T) -> u32 {
        *animal.weight_mut() *= 2;
        animal.be_noisy();
        T::static_noise();
        *animal.weight()
    }

    assert_eq!(measure_weight_twice(&mut duck), 8);
    assert_eq!(measure_weight_twice(&mut dog), 4);
}

#[test]
fn trait_object() {
    struct Dog {
        weight: u32,
        _name: String,
    }

    impl Dog {
        fn be_noisy(&self) {
            println!("Wouaf");
        }
    }

    struct Duck {
        weight: u32
    }

    impl Duck {
        fn be_noisy(&self) {
            println!("Quack");
        }
    }

    let mut duck = Duck { weight: 4 };
    let mut dog = Dog { weight: 2, _name: "".to_string() };

    duck_trait!(Animal, [Dog, Duck], [weight: u32;], [fn be_noisy(&self);]);

    fn measure_weight_twice(animal: &mut dyn Animal) -> u32 {
        *animal.weight_mut() *= 2;
        animal.be_noisy();
        *animal.weight()
    }

    assert_eq!(measure_weight_twice(&mut duck), 8);
    assert_eq!(measure_weight_twice(&mut dog), 4);
}
