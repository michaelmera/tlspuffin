window.SIDEBAR_ITEMS = {"enum":[["ConstFeedback","The [`ConstFeedback`] reports the same value, always. It can be used to enable or disable feedback results through composition."]],"mod":[["concolic","Concolic feedback for concolic fuzzing. It is used to attach concolic tracing metadata to the testcase. This feedback should be used in combination with another feedback as this feedback always considers testcases to be not interesting. Requires a [`ConcolicObserver`] to observe the concolic trace."],["differential","Diff Feedback, comparing the content of two observers of the same type."],["map","Map feedback, maximizing or minimizing maps, for example the afl-style map observer."],["new_hash_feedback","The `NewHashFeedback` uses the backtrace hash and a hashset to only keep novel cases"]],"struct":[["CombinedFeedback","A combined feedback consisting of multiple [`Feedback`]s"],["CrashFeedback","A [`CrashFeedback`] reports as interesting if the target crashed."],["DefaultFeedbackFactory","A feedback factory which merely invokes `::default()` for the feedback type provided"],["ListFeedback","Consider interesting a testcase if the list in `ListObserver` is not empty."],["LogicEagerAnd","Eager `AND` combination of two feedbacks"],["LogicEagerOr","Eager `OR` combination of two feedbacks"],["LogicFastAnd","Fast `AND` combination of two feedbacks"],["LogicFastOr","Fast `OR` combination of two feedbacks"],["NotFeedback","Compose feedbacks with an `NOT` operation"],["TimeFeedback","Nop feedback that annotates execution time in the new testcase, if any for this Feedback, the testcase is never interesting (use with an OR). It decides, if the given [`TimeObserver`] value of a run is interesting."],["TimeoutFeedback","A [`TimeoutFeedback`] reduces the timeout value of a run."]],"trait":[["Feedback","Feedbacks evaluate the observers. Basically, they reduce the information provided by an observer to a value, indicating the “interestingness” of the last run."],["FeedbackFactory","Factory for feedbacks which should be sensitive to an existing context, e.g. observer(s) from a specific execution"],["FeedbackLogic","Logical combination of two feedbacks"],["HasObserverName","Has an associated observer name (mostly used to retrieve the observer with `MatchName` from an `ObserverTuple`)"]],"type":[["CrashFeedbackFactory","A feedback factory for crash feedbacks"],["EagerAndFeedback","Combine two feedbacks with an eager AND operation, will call all feedbacks functions even if not necessary to conclude the result"],["EagerOrFeedback","Combine two feedbacks with an eager OR operation, will call all feedbacks functions even if not necessary to conclude the result"],["FastAndFeedback","Combine two feedbacks with an fast AND operation, might skip calling feedbacks functions if not necessary to conclude the result"],["FastOrFeedback","Combine two feedbacks with an fast OR operation, might skip calling feedbacks functions if not necessary to conclude the result. This means any feedback that is not first might be skipped, use caution when using with `TimeFeedback`"],["TimeoutFeedbackFactory","A feedback factory for timeout feedbacks"]]};