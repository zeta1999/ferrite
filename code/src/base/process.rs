
/// A process / session type. This can be used as either input or output process.
pub trait Process {
  type Value : Sized + Send;
}
