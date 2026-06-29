#[allow(non_upper_case_globals)]
const _3tuple_ref: &[(&str, &str, &str)] = &[
    ("foo", "oof", "ofo"),
    ("bar", "rab", "arb"),
    ("baz", "zab", "abz"),
    ("qux", "xuq", "uqx"),
];
#[allow(non_upper_case_globals)]
const _2tuple_ref: &[(&str, &str)] = &[
    ("foo", "oof"),
    ("bar", "rab"),
    ("baz", "zab"),
    ("qux", "xuq"),
];

pub mod by_macro {

    ///  `ftup!(X::Y::foo, Z::bar, &random_fn)`
    ///
    /// expands to a closure like
    ///
    ///  `|t|
    ///     (X::Y::foo(t.0), Z::bar(t.1), random_fn(&t.2))
    ///   `
    macro_rules! ftup {
        ($(
            $(& $([$($_:tt)* $b:tt])?)?
            $p:path
        ),+)  => {
            |t| (
                $(
                    $p(
                        $($($b)? &)?t.${index()})
                ),+
            )
        };
    }

    fn _demonstrate() {
        use super::{_2tuple_ref, _3tuple_ref};

        let _3tuple_macro_into_owned: Vec<(String, String, String)> = _3tuple_ref
            .into_iter()
            .map(ftup!(Into::into, Into::into, Into::into))
            .collect();
        let _3tuple_macro_to_variety: Vec<_> = _3tuple_macro_into_owned
            .into_iter()
            .map(ftup! {&str::to_uppercase, &String::capacity, String::into_bytes})
            .collect();
        let _2tuple_macro_into_owned: Vec<(String, String)> = _2tuple_ref
            .into_iter()
            .map(ftup!(Into::into, Into::into))
            .collect();
        let _2tuple_macro_to_variety: Vec<_> = _2tuple_macro_into_owned
            .clone()
            .into_iter()
            .map(ftup!(String::into_bytes, arbitrary_op))
            .collect();

        fn arbitrary_op(s: String) -> f64 {
            s.char_indices()
                .fold(0.0f64, |acc, (i, c)| acc + ((c as u32 as f64) / i as f64))
        }
    }
}

pub mod by_fn {
    pub fn spread_3<A, B, C, T, U, V>(
        (af, bf, cf): &(impl Fn(A) -> T, impl Fn(B) -> U, impl Fn(C) -> V),
    ) -> impl Fn((A, B, C)) -> (T, U, V) {
        |(ap, bp, cp): (A, B, C)| -> (T, U, V) { (af(ap), bf(bp), cf(cp)) }
    }

    pub fn spread_3_mut<A, B, C, T, U, V>(
        (mut af, mut bf, mut cf): (impl FnMut(A) -> T, impl FnMut(B) -> U, impl FnMut(C) -> V),
    ) -> impl FnMut((A, B, C)) -> (T, U, V) {
        move |(ap, bp, cp): (A, B, C)| -> (T, U, V) { (af(ap), bf(bp), cf(cp)) }
    }

    pub fn spread_2<A, Ap, Ar, B, Bp, Br>((af, bf): &(A, B)) -> impl Fn((Ap, Bp)) -> (Ar, Br)
    where
        A: Fn(Ap) -> Ar + 'static,
        B: Fn(Bp) -> Br + 'static,
    {
        |(ap, bp): (Ap, Bp)| -> (Ar, Br) { (af(ap), bf(bp)) }
    }

    pub fn spread_2_mut<A, Ap, Ar, B, Bp, Br>(
        (mut af, mut bf): (A, B),
    ) -> impl FnMut((Ap, Bp)) -> (Ar, Br)
    where
        A: FnMut(Ap) -> Ar,
        B: FnMut(Bp) -> Br,
    {
        move |(ap, bp): (Ap, Bp)| -> (Ar, Br) { (af(ap), bf(bp)) }
    }

    pub fn spread_2_once<A, Ap, Ar, B, Bp, Br>(
        (af, bf): (A, B),
    ) -> impl FnOnce((Ap, Bp)) -> (Ar, Br)
    where
        A: FnOnce(Ap) -> Ar,
        B: FnOnce(Bp) -> Br,
    {
        |(ap, bp): (Ap, Bp)| -> (Ar, Br) { (af(ap), bf(bp)) }
    }

    fn _demonstrate() {
        use super::{_2tuple_ref, _3tuple_ref};

        let verbose_vec_3tuple_owned: Vec<(String, String, String)> = _3tuple_ref
            .to_vec()
            .into_iter()
            .map(|(a, b, c)| [a.into(), b.into(), c.into()].into())
            .collect();
        let _func_vec_3tuple_owned: Vec<(String, String, String)> = _3tuple_ref
            .to_vec()
            .into_iter()
            .map(spread_3(&(str::to_string, str::to_string, str::to_string)))
            .collect();

        let _verbose_vec_3tuple_variety: Vec<_> = verbose_vec_3tuple_owned
            .clone()
            .into_iter()
            .map(|(a, b, c)| (a.to_uppercase(), b.capacity(), c.into_bytes()))
            .collect();

        let _func_vec_3tuple_variety: Vec<_> = verbose_vec_3tuple_owned
            .clone()
            .into_iter()
            .map(spread_3_mut((
                |s: String| str::to_uppercase(&*s),
                |s: String| String::capacity(&s),
                |s: String| String::into_bytes(s.into()),
            )))
            .collect();

        let _verbose_vec_2tuple: Vec<_> = _2tuple_ref
            .into_iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let _func_vec_2tuple: Vec<(String, String)> = _2tuple_ref
            .to_vec()
            .into_iter()
            .map(spread_2(&(Into::into, Into::into)))
            .collect();

        let vec_1tuple_ref: Vec<_> = _2tuple_ref.to_vec().into_iter().map(|(a, _)| a).collect();

        let _vec_1tuple_o: Vec<String> =
            vec_1tuple_ref.clone().into_iter().map(Into::into).collect();
    }
}

pub mod by_impl {
    use std::{
        any::{type_name, type_name_of_val},
        marker::PhantomData,
    };

    pub trait TwoTupleExt<A, Ap, Ar, B, Bp, Br>
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        fn into_fn(self) -> TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br>;
    }

    pub struct TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br>
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        pub fns: (A, B),
        ap: PhantomData<Ap>,
        bp: PhantomData<Bp>,
        ar: PhantomData<Ar>,
        br: PhantomData<Br>,
    }

    impl<A, Ap, Ar, B, Bp, Br> TwoTupleExt<A, Ap, Ar, B, Bp, Br> for (A, B)
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        fn into_fn(self) -> TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br> {
            TwoTupleExtStruct {
                fns: self,
                ap: PhantomData,
                bp: PhantomData,
                ar: PhantomData,
                br: PhantomData,
            }
        }
    }

    impl<A, Ap, Ar, B, Bp, Br> FnOnce<((Ap, Bp),)> for TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br>
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        type Output = (Ar, Br);
        extern "rust-call" fn call_once(self, (args,): ((Ap, Bp),)) -> Self::Output {
            (self.fns.0(args.0), self.fns.1(args.1))
        }
    }

    impl<A, Ap, Ar, B, Bp, Br> FnMut<((Ap, Bp),)> for TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br>
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        extern "rust-call" fn call_mut(&mut self, (args,): ((Ap, Bp),)) -> Self::Output {
            (self.fns.0(args.0), self.fns.1(args.1))
        }
    }

    impl<A, Ap, Ar, B, Bp, Br> Fn<((Ap, Bp),)> for TwoTupleExtStruct<A, Ap, Ar, B, Bp, Br>
    where
        A: Fn(Ap) -> Ar,
        B: Fn(Bp) -> Br,
    {
        extern "rust-call" fn call(&self, (args,): ((Ap, Bp),)) -> Self::Output {
            (self.fns.0(args.0), self.fns.1(args.1))
        }
    }

    fn _demonstrate() {
        let t = (str::to_uppercase, str::to_uppercase);
        let f = t.into_fn();
        let s = f(("abc", "def"));
        assert_eq!(type_name_of_val(&s), type_name::<(String, String)>());

        use super::_2tuple_ref;

        let _vr = _2tuple_ref
            .to_vec()
            .into_iter()
            .map((str::to_uppercase, str::to_lowercase).into_fn())
            .collect::<Vec<_>>();

        // not ideal, as we would prefer to write
        //  let va = _2tuple_ref
        //      .into_iter()
        //      .map((str::to_uppercase, str::to_uppercase))
        //      .collect::<Vec<_>>();
    }

    // use std::marker::Tuple;
    // pub trait TupleExt {}

    // impl<T: Tuple> Fn<T> for TupleExt<T> {
    //     extern "rust-call" fn call(&self, args: T) -> Self::Output {
    //         for f in self {}
    //     }
    // }

    // pub struct TupleExtStruct {
    //     pub fns: Tuple,
    // }

    // pub trait TwoTupleExtOnce<A, Ap, Ar, B, Bp, Br>
    // where
    //     A: FnOnce(Ap) -> Ar,
    //     B: FnOnce(Bp) -> Br,
    // {
    // }
    // pub trait TwoTupleExtMut<A, Ap, Ar, B, Bp, Br>
    // where
    //     A: FnMut(Ap) -> Ar,
    //     B: FnMut(Bp) -> Br,
    // {
    // }
}

// mod first_class_tuples {

//     // use std::marker::{PhantomData, Tuple};

//     #[allow(dead_code, unused)]
//     pub trait TupleExt {
//         fn hello_world(&self) {}
//     }

//     impl<T> TupleExt for T
//     where
//         // T: Tuple,
//     {
//         fn hello_world(&self) {}
//     }

//     #[allow(dead_code, unused)]
//     fn foo() {
//         let bar = ("a", 2);
//         bar.hello_world();
//         (3, 2, "yes").hello_world();
//     }

//     #[allow(dead_code, unused)]
//     fn foobar() {
//         let r1 = vec!["1", "2", "3"];
//         let r2 = vec![("1", "2"), ("3", "4"), ("5", "6")];
//         let r3 = vec![("1", "2", "3"), ("4", "5", "6"), ("7", "8", "9")];

//         let v1 = r1
//             .clone()
//             .into_iter()
//             .map(|a| String::from(a))
//             .collect::<Vec<_>>();
//         let s1 = r1.clone().into_iter().map(String::from).collect::<Vec<_>>();

//         let v2 = r2
//             .clone()
//             .into_iter()
//             .map(|(a, b)| (String::from(a), String::from(b)))
//             .collect::<Vec<_>>();
//         let s2 = r2
//             .clone()
//             .into_iter()
//             // .map(((String::from, String::from)).as_fn())
//             .collect::<Vec<_>>();
//         let h2 = r2
//             .clone()
//             .into_iter()
//             .map(CallTwo {
//                 a: String::from,
//                 b: String::from,
//                 outputs: PhantomData,
//             })
//             .collect::<Vec<_>>();
//         // let v2 = r2.map((String::from, String::from));

//         // let t2 = Some('2').map(CallTwo { fns: (String::from, String::from), outputs: PhantomData })
//     }

//     #[allow(dead_code, unused)]
//     pub struct CallTwo<T, R, U, V, A, B>
//     where
//         A: FnOnce(T) -> R,
//         B: FnOnce(U) -> V,
//     {
//         a: A,
//         b: B,
//         outputs: PhantomData<(A::Output, B::Output)>,
//     }

//     impl<T, R, U, V, A, B> FnOnce<((T, U),)> for CallTwo<T, R, U, V, A, B>
//     where
//         A: FnOnce(T) -> R,
//         B: FnOnce(U) -> V,
//     {
//         type Output = (A::Output, B::Output);
//         extern "rust-call" fn call_once(self, args: ((T, U),)) -> Self::Output {
//             let Self { a, b, .. } = self;
//             let ((arg_a, arg_b),): ((T, U),) = args;
//             (a(arg_a), b(arg_b))
//         }
//     }

//     impl<T, R, U, V, A, B> FnMut<((T, U),)> for CallTwo<T, R, U, V, A, B>
//     where
//         A: FnMut(T) -> R,
//         B: FnMut(U) -> V,
//     {
//         extern "rust-call" fn call_mut(&mut self, args: ((T, U),)) -> Self::Output {
//             let Self { a, b, .. } = self;
//             let ((arg_a, arg_b),): ((T, U),) = args;
//             (a(arg_a), b(arg_b))
//         }
//     }

//     #[allow(dead_code, unused)]
//     pub trait FunTuple<T, R, U, V, A, B>
//     where
//         A: FnOnce(T) -> R,
//         B: FnOnce(U) -> V,
//     {
//         type Output;

//         fn as_fn(self) -> CallTwo<T, R, U, V, A, B>;
//     }

//     impl<T, R, U, V, A, B> FunTuple<T, A::Output, U, B::Output, A, B> for (A, B)
//     where
//         T: Tuple,
//         U: Tuple,
//         A: FnOnce(T) -> R,
//         B: FnOnce(U) -> V,
//     {
//         type Output = (A::Output, B::Output);

//         fn as_fn(self) -> CallTwo<T, A::Output, U, B::Output, A, B> {
//             let (a, b): (A, B) = self;
//             CallTwo {
//                 a,
//                 b,
//                 outputs: PhantomData,
//             }
//         }
//     }

//     #[allow(dead_code, unused)]
//     pub trait FunTupleMut<T, R, U, V, A, B>
//     where
//         A: FnMut(T) -> R,
//         B: FnMut(U) -> V,
//     {
//         type Output;
//         type FunBecome;

//         fn as_fn(self) -> Self::FunBecome;
//     }

//     impl<T, U, A, B> FunTupleMut<T, A::Output, U, B::Output, A, B> for (A, B)
//     where
//         T: Tuple,
//         U: Tuple,
//         A: FnMut(T),
//         B: FnMut(U),
//     {
//         type Output = (A::Output, B::Output);
//         type FunBecome = CallTwo<T, A::Output, U, B::Output, A, B>;

//         fn as_fn(self) -> Self::FunBecome {
//             let (a, b): (A, B) = self;
//             CallTwo {
//                 a,
//                 b,
//                 outputs: PhantomData,
//             }
//         }
//     }

//     // impl<A, B, C> JoinTrait for (A, B, C)
//     // where
//     //     A: IntoFuture,
//     //     B: IntoFuture,
//     //     C: IntoFuture,
//     // {
//     //     type Output = (A::Output, B::Output, C::Output);
//     //     type Future = Join3<A::IntoFuture, B::IntoFuture, C::IntoFuture>;
//     //     fn join(self) -> Self::Future {
//     //         let (A, B, C): (A, B, C) = self;
//     //         Join3 {
//     //             futures: join3::Futures {
//     //                 A: ManuallyDrop::new(A.into_future()),
//     //                 B: ManuallyDrop::new(B.into_future()),
//     //                 C: ManuallyDrop::new(C.into_future()),
//     //             },
//     //             state: PollArray::new_pending(),
//     //             outputs: (
//     //                 MaybeUninit::<A::Output>::uninit(),
//     //                 MaybeUninit::<B::Output>::uninit(),
//     //                 MaybeUninit::<C::Output>::uninit(),
//     //             ),
//     //             wakers: WakerArray::new(),
//     //             completed: 0,
//     //         }
//     //     }
//     // }

//     // // macro_rules! impl_join_tuple {
//     // //     ($mod_name:ident $StructName:ident) => {
//     // //         /// This `struct` is created by the [`join`] method on the [`Join`] trait. See
//     // //         /// its documentation for more.
//     // //         #[allow(non_snake_case)]
//     // //         pub struct $StructName {}

//     // //         impl fmt::Debug for $StructName {
//     // //             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     // //                 f.debug_tuple("Join").finish()
//     // //             }
//     // //         }

//     // //         impl Future for $StructName {
//     // //             type Output = ();

//     // //             fn poll(
//     // //                 self: Pin<&mut Self>, _cx: &mut Context<'_>
//     // //             ) -> Poll<Self::Output> {
//     // //                 Poll::Ready(())
//     // //             }
//     // //         }

//     // //         impl JoinTrait for () {
//     // //             type Output = ();
//     // //             type Future = $StructName;
//     // //             fn join(self) -> Self::Future {
//     // //                 $StructName {}
//     // //             }
//     // //         }
//     // //     };
//     // //     ($mod_name:ident $StructName:ident $($F:ident)+) => {
//     // //         mod $mod_name {
//     // //             use core::mem::ManuallyDrop;

//     // //             #[pin_project::pin_project]
//     // //             pub(super) struct Futures<$($F,)+> {$(
//     // //                 #[pin]
//     // //                 pub(super) $F: ManuallyDrop<$F>,
//     // //             )+}

//     // //             #[repr(u8)]
//     // //             pub(super) enum Indexes { $($F,)+ }

//     // //             pub(super) const LEN: usize = [$(Indexes::$F,)+].len();
//     // //         }

//     // //         /// Waits for many similarly-typed futures to complete.
//     // //         ///
//     // //         /// This `struct` is created by the [`join`] method on the [`Join`] trait. See
//     // //         /// its documentation for more.
//     // //         ///
//     // //         /// [`join`]: crate::future::Join::join
//     // //         /// [`Join`]: crate::future::Join
//     // //         #[pin_project(PinnedDrop)]
//     // //         #[must_use = "futures do nothing unless you `.await` or poll them"]
//     // //         #[allow(non_snake_case)]
//     // //         pub struct $StructName<$($F: Future),+> {
//     // //             #[pin]
//     // //             futures: $mod_name::Futures<$($F,)+>,
//     // //             outputs: ($(MaybeUninit<$F::Output>,)+),
//     // //             // trace the state of outputs, marking them as ready or consumed
//     // //             // then, drop the non-consumed values, if any
//     // //             state: PollArray<{$mod_name::LEN}>,
//     // //             wakers: WakerArray<{$mod_name::LEN}>,
//     // //             completed: usize,
//     // //         }

//     // //         impl<$($F),+> Debug for $StructName<$($F),+>
//     // //         where
//     // //             $( $F: Future + Debug, )+
//     // //         {
//     // //             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     // //                 f.debug_tuple("Join")
//     // //                     $(.field(&self.futures.$F))+
//     // //                     .finish()
//     // //             }
//     // //         }

//     // //         #[allow(unused_mut)]
//     // //         #[allow(unused_parens)]
//     // //         #[allow(unused_variables)]
//     // //         impl<$($F: Future),+> Future for $StructName<$($F),+> {
//     // //             type Output = ($($F::Output,)+);

//     // //             fn poll(
//     // //                 self: Pin<&mut Self>, cx: &mut Context<'_>
//     // //             ) -> Poll<Self::Output> {
//     // //                 const LEN: usize = $mod_name::LEN;

//     // //                 let mut this = self.project();
//     // //                 let all_completed = !(*this.completed == LEN);
//     // //                 assert!(all_completed, "Futures must not be polled after completing");

//     // //                 let mut futures = this.futures.project();

//     // //                 let mut readiness = this.wakers.readiness();
//     // //                 readiness.set_waker(cx.waker());

//     // //                 for index in 0..LEN {
//     // //                     if !readiness.any_ready() {
//     // //                         // nothing ready yet
//     // //                         return Poll::Pending;
//     // //                     }
//     // //                     if !readiness.clear_ready(index) || this.state[index].is_ready() {
//     // //                         // future not ready yet or already polled to completion, skip
//     // //                         continue;
//     // //                     }

//     // //                     // unlock readiness so we don't deadlock when polling
//     // //                     #[allow(clippy::drop_non_drop)]
//     // //                     drop(readiness);

//     // //                     // obtain the intermediate waker
//     // //                     let mut cx = Context::from_waker(this.wakers.get(index).unwrap());

//     // //                     // generate the needed code to poll `futures.{index}`
//     // //                     // SAFETY: the future's state should be "pending", so it's safe to poll
//     // //                     unsafe_poll!(index, this, futures, cx, LEN, $($F,)+);

//     // //                     if *this.completed == LEN {
//     // //                         let out = {
//     // //                             let mut out = ($(MaybeUninit::<$F::Output>::uninit(),)+);
//     // //                             core::mem::swap(&mut out, this.outputs);
//     // //                             let ($($F,)+) = out;
//     // //                             unsafe { ($($F.assume_init(),)+) }
//     // //                         };

//     // //                         this.state.set_all_none();

//     // //                         return Poll::Ready(out);
//     // //                     }
//     // //                     readiness = this.wakers.readiness();
//     // //                 }

//     // //                 Poll::Pending
//     // //             }
//     // //         }

//     // //         #[pinned_drop]
//     // //         impl<$($F: Future),+> PinnedDrop for $StructName<$($F),+> {
//     // //             fn drop(self: Pin<&mut Self>) {
//     // //                 let this = self.project();

//     // //                 let &mut ($(ref mut $F,)+) = this.outputs;

//     // //                 let states = this.state;
//     // //                 let mut futures = this.futures;
//     // //                 drop_initialized_values!($($F,)+ | states);
//     // //                 drop_pending_futures!(states, futures, $($F,)+);
//     // //             }
//     // //         }

//     // //         #[allow(unused_parens)]
//     // //         impl<$($F),+> JoinTrait for ($($F,)+)
//     // //         where $(
//     // //             $F: IntoFuture,
//     // //         )+ {
//     // //             type Output = ($($F::Output,)*);
//     // //             type Future = $StructName<$($F::IntoFuture),*>;

//     // //             fn join(self) -> Self::Future {
//     // //                 let ($($F,)+): ($($F,)+) = self;
//     // //                 $StructName {
//     // //                     futures: $mod_name::Futures {$($F: ManuallyDrop::new($F.into_future()),)+},
//     // //                     state: PollArray::new_pending(),
//     // //                     outputs: ($(MaybeUninit::<$F::Output>::uninit(),)+),
//     // //                     wakers: WakerArray::new(),
//     // //                     completed: 0,
//     // //                 }
//     // //             }
//     // //         }
//     // //     };
//     // // }

//     // // impl_join_tuple! { join0 Join0 }
//     // // impl_join_tuple! { join1 Join1 A }
//     // // impl_join_tuple! { join2 Join2 A B }
//     // // impl_join_tuple! { join3 Join3 A B C }
//     // // impl_join_tuple! { join4 Join4 A B C D }
//     // // impl_join_tuple! { join5 Join5 A B C D E }
//     // // impl_join_tuple! { join6 Join6 A B C D E F }
//     // // impl_join_tuple! { join7 Join7 A B C D E F G }
//     // // impl_join_tuple! { join8 Join8 A B C D E F G H }
//     // // impl_join_tuple! { join9 Join9 A B C D E F G H I }
//     // // impl_join_tuple! { join10 Join10 A B C D E F G H I J }
//     // // impl_join_tuple! { join11 Join11 A B C D E F G H I J K }
//     // // impl_join_tuple! { join12 Join12 A B C D E F G H I J K L }
//     // // impl_join_tuple! { join13 Join13 A B C D E F G H I J K L M }
//     // // impl_join_tuple! { join14 Join14 A B C D E F G H I J K L M N }
//     // // impl_join_tuple! { join15 Join15 A B C D E F G H I J K L M N O }
// }
