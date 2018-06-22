use dispatcher::operation::array_handler::ArrayHandler;
use dispatcher::operation::function::FunctionHolder;
use optimiser::loop_optimiser;

#[derive(Clone, Debug)]
pub enum Operation {
    Add(i32),
    Move(i32),
    MoveTo(Vec<(i32, i32)>),
    Set(i32),
    Read,
    Write,
    While(Vec<Operation>),
    InsFuns,
    CallFun,
    CallFSep(usize),
    Debug,
    EmptyOp,
}

impl Operation {
    pub fn execute(& self, ah: &mut ArrayHandler, fun_holder: &FunctionHolder) {
        match self {
            &Operation::Add(i) => ah.add(i),
            &Operation::Move(i) => ah.move_r(i),
            &Operation::Set(i) => ah.set(i),
            &Operation::While(ref ops) => {
                while ah.get() != 0 {
                    for op in ops.iter() {
                        op.execute(ah, fun_holder);
                    }
                }
            }
            &Operation::InsFuns => ah.set(fun_holder.no_functions() as i32 ),
            &Operation::CallFun => fun_holder.execute(ah),
            &Operation::CallFSep(args) => fun_holder.execute_separate(ah, args),
            &Operation::Read => ah.read(),
            &Operation::Write => ah.write(),
            &Operation::Debug => ah.debug(fun_holder.no_functions()),
            &Operation::MoveTo(ref places) => {
                let val = ah.get();
                ah.set(0);
                for &(place, mult) in places.iter() {
                    ah.add_at(place, val*mult);
                }
            }
            &Operation::EmptyOp => panic!("Empty operation run!"),
        }
    }
}

pub fn new_while(ops: Vec<Operation>) -> Operation {
    loop_optimiser(ops)
}