pub trait Factory<I, O> {
    type Config;
    fn gen(&'static self, input: I) -> O;
}


