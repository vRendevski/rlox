use super::Value;

pub enum ControlSignal {
  None,
  Return(Value),
}
