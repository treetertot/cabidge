use cabidge_code::{func::Value, module::Reference};

pub enum Op<F, A> {
    Terminate(Value),
    // Any I/O operators will also need a continuation parameter
    //Debug(Value),
    //LoadPath(Value),
    LoadFunc(Reference<F>),
    LoadAtom(Reference<A>),
    Apply {
        f: Value,
        args: Vec<Value>
    }
}