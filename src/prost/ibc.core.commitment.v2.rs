// This file is @generated by prost-build.
/// MerklePath is the path used to verify commitment proofs, which can be an
/// arbitrary structured object (defined by a commitment type).
/// MerklePath is represented from root-to-leaf
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub key_path: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for MerklePath {
    const NAME: &'static str = "MerklePath";
    const PACKAGE: &'static str = "ibc.core.commitment.v2";
    fn full_name() -> ::prost::alloc::string::String {
        "ibc.core.commitment.v2.MerklePath".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/ibc.core.commitment.v2.MerklePath".into()
    }
}
