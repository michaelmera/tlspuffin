window.SIDEBAR_ITEMS = {"fn":[["trace_mutations",""]],"mod":[["util",""]],"struct":[["GenerateMutator","GENERATE: Generates a previously-unseen term using a term zoo"],["RemoveAndLiftMutator","REMOVE AND LIFT: Removes a sub-term from a term and attaches orphaned children to the parent (such that types match). This only works if there is only a single child."],["RepeatMutator","REPEAT: Repeats an input which is already part of the trace"],["ReplaceMatchMutator","REPLACE-MATCH: Replaces a function symbol with a different one (such that types match). An example would be to replace a constant with another constant or the binary function fn_add with fn_sub. It can also replace any variable with a constant."],["ReplaceReuseMutator","REPLACE-REUSE: Replaces a sub-term with a different sub-term which is part of the trace (such that types match). The new sub-term could come from another step which has a different recipe term."],["SkipMutator","SKIP:  Removes an input step"],["SwapMutator","SWAP: Swaps a sub-term with a different sub-term which is part of the trace (such that types match)."]]};