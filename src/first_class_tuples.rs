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
