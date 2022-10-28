#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(enumeration="ActionState", tag="2")]
    pub state: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionState {
    Start = 0,
    Stop = 1,
    End = 2,
    Continue = 3,
}
impl ActionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionState::Start => "Start",
            ActionState::Stop => "Stop",
            ActionState::End => "End",
            ActionState::Continue => "Continue",
        }
    }
}
