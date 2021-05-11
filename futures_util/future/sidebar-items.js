initSidebarItems({"enum":[["Either","Combines two different futures, streams, or sinks having the same associated types into a single type."],["MaybeDone","A future that may have completed."],["TryMaybeDone","A future that may have completed with an error."]],"fn":[["abortable","Creates a new `Abortable` future and an `AbortHandle` which can be used to stop it."],["err","Create a future that is immediately ready with an error value."],["join","Joins the result of two futures, waiting for them both to complete."],["join3","Same as `join`, but with more futures."],["join4","Same as `join`, but with more futures."],["join5","Same as `join`, but with more futures."],["join_all","Creates a future which represents a collection of the outputs of the futures given."],["lazy","Creates a new future that allows delayed execution of a closure."],["maybe_done","Wraps a future into a `MaybeDone`"],["ok","Create a future that is immediately ready with a success value."],["pending","Creates a future which never resolves, representing a computation that never finishes."],["poll_fn","Creates a new future wrapping around a function returning [`Poll`]."],["ready","Creates a future that is immediately ready with a value."],["select","Waits for either one of two differently-typed futures to complete."],["select_all","Creates a new future which will select over a list of futures."],["select_ok","Creates a new future which will select the first successful future over a list of futures."],["try_join","Joins the result of two futures, waiting for them both to complete or for one to produce an error."],["try_join3","Same as `try_join`, but with more futures."],["try_join4","Same as `try_join`, but with more futures."],["try_join5","Same as `try_join`, but with more futures."],["try_join_all","Creates a future which represents either a collection of the results of the futures given or an error."],["try_maybe_done","Wraps a future into a `TryMaybeDone`"],["try_select","Waits for either one of two differently-typed futures to complete."]],"struct":[["AbortHandle","A handle to an `Abortable` task."],["AbortRegistration","A registration handle for an `Abortable` task. Values of this type can be acquired from `AbortHandle::new` and are used in calls to `Abortable::new`."],["Abortable","A future/stream which can be remotely short-circuited using an `AbortHandle`."],["Aborted","Indicator that the `Abortable` task was aborted."],["AndThen","Future for the `and_then` method."],["CatchUnwind","Future for the `catch_unwind` method."],["ErrInto","Future for the `err_into` method."],["Flatten","Future for the `flatten` method."],["FlattenStream","Stream for the `flatten_stream` method."],["Fuse","Future for the `fuse` method."],["FutureObj","A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + Send + 'a>`."],["Inspect","Future for the `inspect` method."],["InspectErr","Future for the `inspect_err` method."],["InspectOk","Future for the `inspect_ok` method."],["IntoFuture","Future for the `into_future` method."],["IntoStream","Stream for the `into_stream` method."],["Join","Future for the `join` function."],["Join3","Future for the [`join3`] function."],["Join4","Future for the [`join4`] function."],["Join5","Future for the [`join5`] function."],["JoinAll","Future for the [`join_all`] function."],["Lazy","Future for the [`lazy`] function."],["LocalFutureObj","A custom trait object for polling futures, roughly akin to `Box<dyn Future<Output = T> + 'a>`."],["Map","Future for the `map` method."],["MapErr","Future for the `map_err` method."],["MapInto","Future for the `map_into` combinator."],["MapOk","Future for the `map_ok` method."],["MapOkOrElse","Future for the `map_ok_or_else` method."],["NeverError","Future for the `never_error` combinator."],["OkInto","Future for the `ok_into` method."],["OptionFuture","A future representing a value which may or may not be present."],["OrElse","Future for the `or_else` method."],["Pending","Future for the [`pending()`] function."],["PollFn","Future for the [`poll_fn`] function."],["Ready","Future for the `ready` function."],["Select","Future for the [`select()`] function."],["SelectAll","Future for the [`select_all`] function."],["SelectOk","Future for the [`select_ok`] function."],["Shared","Future for the `shared` method."],["Then","Future for the `then` method."],["TryFlatten","Future for the `try_flatten` method."],["TryFlattenStream","Future for the `try_flatten_stream` method."],["TryJoin","Future for the `try_join` function."],["TryJoin3","Future for the [`try_join3`] function."],["TryJoin4","Future for the [`try_join4`] function."],["TryJoin5","Future for the [`try_join5`] function."],["TryJoinAll","Future for the [`try_join_all`] function."],["TrySelect","Future for the [`try_select()`] function."],["UnitError","Future for the `unit_error` combinator."],["UnwrapOrElse","Future for the `unwrap_or_else` method."],["WeakShared","A weak reference to a [`Shared`] that can be upgraded much like an `Arc`."]],"trait":[["FusedFuture","A future which tracks whether or not the underlying future should no longer be polled."],["FutureExt","An extension trait for `Future`s that provides a variety of convenient adapters."],["TryFuture","A convenience for futures that return `Result` values that includes a variety of adapters tailored to such futures."],["TryFutureExt","Adapters specific to [`Result`]-returning futures"],["UnsafeFutureObj","A custom implementation of a future trait object for `FutureObj`, providing a vtable with drop support."]],"type":[["BoxFuture","An owned dynamically typed [`Future`] for use in cases where you can't statically type your result or need to add some indirection."],["LocalBoxFuture","`BoxFuture`, but without the `Send` requirement."]]});