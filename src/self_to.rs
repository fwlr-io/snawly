/// This experiment works for
///     let baz = String::from("bar").to(|s| s.replace("r", "z"));
///
/// It does not work for
///     let baz = String::from("bar").to(|s| Regex::new("r").unwrap().replace_all(s, "z"));
/// But that's because of the behaviour of Cow. This works
///     let baz = String::from("bar").to(|s| Regex::new("r").unwrap().replace_all(s, "z").into_owned());
///
/// However, this experiment still doesn't work for
///     let bar = String::from("bar").to(String::from);
/// It gives the error
///     implementation of FnOnce is not general enough
///       fn(&'2 String) -> String  must implement FnOnce<(&'1 String,)>, for any lifetime '1…
///       …but it actually implements FnOnce<(&'2 String,)>, for some specific lifetime '2 (rustc)
///
/// This is unfortunately currently beyond my Rust knowledge, so it's tabled for now.
///
/// With some more work, I now have the below, which gives
///     let term = term.to(|s| s.replace(PARENT_STYLE, PARENT_CLASS));
///     let term = term.to(|s| DIV_CLOSE.replace_all(&s, SPAN_CLOSE));
///         "requires that '1 outlive '2"
/// I suspect I need to use https://doc.rust-lang.org/nightly/reference/trait-bounds.html?highlight=outlive#r-bound.lifetime
/// fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
///    y = x;
///    let r: &'b &'a i32 = &&0;

pub trait To {
    fn to<F, T>(self, f: F) -> T
    where
        for<'a> F: FnOnce(&'a Self) -> T + 'a;
}
impl<S> To for S {
    fn to<F, T>(self: S, f: F) -> T
    where
        for<'a> F: FnOnce(&'a S) -> T + 'a,
    {
        f(&self)
    }
}

// A failed attempt at wrangling the lifetime for <Cow ...>
// pub mod self_to {
//     pub trait To<'a> {
//         fn to<T>(&self, f: impl FnOnce(&Self) -> T) -> T
//         where
//             Self: 'a,
//             T: 'a;
//     }
//     impl<S> To<'_> for &'_ S {
//         fn to<T>(&self, f: impl FnOnce(&Self) -> T) -> T {
//             f(self)
//         }
//     }
// }

// A failed attempt at wrangling the lifetime for "FnOnce not general enough"
// pub mod self_to {
//     pub trait To<'a> {
//         fn to<U, F>(&self, f: F) -> U
//         where
//             Self: 'a,
//             U: 'a,
//             F: FnOnce(&Self) -> U;
//     }
//     impl<'a, T> To<'a> for T {
//         fn to<U, F: FnOnce(&Self) -> U>(&self, f: F) -> U {
//             f(self)
//         }
//     }
// }
// More reading: GATs, HKTs?
// https://users.rust-lang.org/t/implementation-is-not-general-enough-is-actually-implemented-for-some-specific-lifetime/109127/3
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6ff0461b517ddd8a3073f3d4b584e915
