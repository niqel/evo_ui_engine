use super::acetate_io::AcetateIO;

pub trait AcetateInit {
    fn new() -> Self
    where
        Self: Sized;

    fn input(&self, io: AcetateIO) -> Self
    where
        Self: Sized;
}
