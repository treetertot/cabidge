use cabidge_code::func::Value;

pub enum Op {
    Terminate,
    Debug(Value),
    LoadStd,
    LoadSelf,
    //LoadPath(Value),

}