fn main() {
    // consume ordering is a lighweight, more effective, variant of acquire ordering
    // synchronizing effects limited to things that depend on the loaded value

    // release store happened before eval of dependant expressions (array[x]), but not necessarily before
    // independent operations (work with some other var)

    // The good
    // - exact same instr as relaxed ordering, can be "free". not the case with acquire
    // The bad
    // - no compiler actually implements consume ordering :(
}
