pub trait Resulty
    where Self: std::fmt::Debug{
    fn to_string(&self) -> String {
        format!("{:?}",&self)
    }
}