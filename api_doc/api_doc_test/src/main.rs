use api_doc::reflect::TypeInfo;
use api_doc_derive::{TypeInfo, method_info};
use api_doc;
use serde_derive::{Serialize, Deserialize};
use api_doc::api::{Method};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StringId (String);
pub type BlockId = StringId;

#[derive(Serialize, Deserialize, TypeInfo)]
pub enum EnumWithValues {
    Foo = 2, Bar
}

#[derive(Serialize, Deserialize, TypeInfo)]
pub enum EnumWithTypes {
    Foo(String, String),
    Bar(u32)
}

#[doc(summary = "Foo")]
/// Foo struct
#[derive(Serialize, Deserialize, TypeInfo, Default)]
pub struct Foo {
    pub address: Option<String>,
    pub message_id: String,
    pub message_body_base64: String,
    pub expire: Option<u32>,
    last_block_id: BlockId,
    sending_time: u32,
}

#[derive(Serialize, Deserialize, TypeInfo, Default)]
struct Bar {
    #[doc(summary = "summary")]
    #[doc = "description"]
    pub abi: Option<serde_json::Value>,
    pub function_name: Option<String>,
    pub message: Foo,
    #[serde(default)]
    pub infinite_wait: bool,
    pub ids: Vec<String>,
    pub arr: [u64; 2],
}

#[derive(Serialize, Deserialize, TypeInfo)]
struct FooHandle(u32);

#[method_info(name = "module.baz")]
/// This is baz method
fn baz(_params: Foo) -> Bar {
    Bar::default()
}

fn reflect<T: TypeInfo>() {
    let info = serde_json::to_string_pretty(&T::type_info()).unwrap();
    println!("{}", info);

}

fn main() {
    reflect::<Foo>();
    reflect::<Bar>();
    reflect::<EnumWithValues>();
    reflect::<EnumWithTypes>();
    reflect::<FooHandle>();
    println!("{:?}", baz_method());

    let _ = baz(Foo::default());
}