window.SIDEBAR_ITEMS = {"enum":[["DiffExitKind","How one of the diffing executions finished."],["ExitKind","How an execution finished."]],"mod":[["combined","A `CombinedExecutor` wraps a primary executor and a secondary one In comparison to the [`crate::executors::DiffExecutor`] it does not run the secondary executor in `run_target`."],["command","The command executor executes a sub program for each run"],["differential","Executor for differential fuzzing. It wraps two exeutors that will be run after each other with the same input. In comparison to the [`crate::executors::CombinedExecutor`] it also runs the secondary executor in `run_target`."],["forkserver","Expose an `Executor` based on a `Forkserver` in order to execute AFL/AFL++ binaries"],["inprocess","The [`InProcessExecutor`] is a libfuzzer-like executor, that will simply call a function. It should usually be paired with extra error-handling, such as a restarting event manager, to be effective."],["shadow","A `ShadowExecutor` wraps an executor to have shadow observer that will not be considered by the feedbacks and the manager"],["timeout","Timeout executor. Not possible on `no-std` Windows or `no-std`, but works for unix A `TimeoutExecutor` sets a timeout before each target run"],["with_observers","A wrapper for any [`Executor`] to make it implement [`HasObservers`] using a given [`ObserversTuple`]."]],"trait":[["Executor","An executor takes the given inputs, and runs the harness/target."],["HasObservers","Holds a tuple of Observers"]]};