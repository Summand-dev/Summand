pub trait Interpreter<InterpreterContext, T> {
    fn interpret(&mut self, context: InterpreterContext) -> T;
}