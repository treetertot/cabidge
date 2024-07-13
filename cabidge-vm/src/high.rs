use cabidge_code::{func::Value, module::Reference};

pub enum Op<F, A> {
    Terminate(Value),
    //Debug(Value),
    //LoadPath(Value),
    LoadFunc(Reference<F>),
    LoadAtom(Reference<A>),
    Apply {
        f: Value,
        args: Vec<Value>
    }
}