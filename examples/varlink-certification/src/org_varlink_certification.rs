//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

use serde_json::{self, Value};
use std::io;
use std::sync::{Arc, RwLock};
use varlink;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Interface {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foo: Option<Vec<Option<varlink::StringHashMap<Interface_foo>>>>,
    pub anon: Interface_anon,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyType {
    pub object: Value,
    #[serde(rename = "enum")] pub enum_: MyType_enum,
    #[serde(rename = "struct")] pub struct_: MyType_struct,
    pub array: Vec<String>,
    pub dictionary: varlink::StringHashMap<String>,
    pub stringset: varlink::StringHashSet,
    #[serde(skip_serializing_if = "Option::is_none")] pub nullable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable_array_struct: Option<Vec<MyType_nullable_array_struct>>,
    pub interface: Interface,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EndReply_ {}

impl varlink::VarlinkReply for EndReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EndArgs_ {
    pub mytype: MyType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StartReply_ {}

impl varlink::VarlinkReply for StartReply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StartArgs_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test01Reply_ {
    pub bool: bool,
}

impl varlink::VarlinkReply for Test01Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test01Args_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test02Reply_ {
    pub int: i64,
}

impl varlink::VarlinkReply for Test02Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test02Args_ {
    pub bool: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test03Reply_ {
    pub float: f64,
}

impl varlink::VarlinkReply for Test03Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test03Args_ {
    pub int: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test04Reply_ {
    pub string: String,
}

impl varlink::VarlinkReply for Test04Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test04Args_ {
    pub float: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test05Reply_ {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

impl varlink::VarlinkReply for Test05Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test05Args_ {
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test06Reply_ {
    #[serde(rename = "struct")] pub struct_: Test06Reply_struct,
}

impl varlink::VarlinkReply for Test06Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test06Args_ {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test07Reply_ {
    pub map: varlink::StringHashMap<String>,
}

impl varlink::VarlinkReply for Test07Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test07Args_ {
    #[serde(rename = "struct")] pub struct_: Test07Args_struct,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test08Reply_ {
    pub set: varlink::StringHashSet,
}

impl varlink::VarlinkReply for Test08Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test08Args_ {
    pub map: varlink::StringHashMap<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test09Reply_ {
    pub mytype: MyType,
}

impl varlink::VarlinkReply for Test09Reply_ {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test09Args_ {
    pub set: varlink::StringHashSet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CertificationErrorArgs_ {
    pub wants: String,
    pub got: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Interface_anon {
    pub foo: bool,
    pub bar: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyType_struct {
    pub first: i64,
    pub second: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyType_nullable_array_struct {
    pub first: i64,
    pub second: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test06Reply_struct {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Test07Args_struct {
    pub bool: bool,
    pub int: i64,
    pub float: f64,
    pub string: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Interface_foo {
    foo,
    bar,
    baz,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MyType_enum {
    one,
    two,
    three,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_certification_error(&mut self, wants: String, got: String) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.varlink.certification.CertificationError".into(),
            Some(serde_json::to_value(CertificationErrorArgs_ { wants, got }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

#[derive(Debug)]
pub enum Error_ {
    CertificationError(Option<CertificationErrorArgs_>),
    VarlinkError_(varlink::Error),
    UnknownError_(varlink::Reply),
    IOError_(io::Error),
    JSONError_(serde_json::Error),
}

impl From<varlink::Reply> for Error_ {
    fn from(e: varlink::Reply) -> Self {
        if varlink::Error::is_error(&e) {
            return Error_::VarlinkError_(e.into());
        }

        match e {
            varlink::Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.certification.CertificationError" =>
            {
                match e {
                    varlink::Reply {
                        parameters: Some(p),
                        ..
                    } => match serde_json::from_value(p) {
                        Ok(v) => Error_::CertificationError(v),
                        Err(_) => Error_::CertificationError(None),
                    },
                    _ => Error_::CertificationError(None),
                }
            }
            _ => return Error_::UnknownError_(e),
        }
    }
}

impl From<io::Error> for Error_ {
    fn from(e: io::Error) -> Self {
        Error_::IOError_(e)
    }
}

impl From<serde_json::Error> for Error_ {
    fn from(e: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match e.classify() {
            Category::Io => Error_::IOError_(e.into()),
            _ => Error_::JSONError_(e),
        }
    }
}

impl From<Error_> for io::Error {
    fn from(e: Error_) -> Self {
        match e {
            Error_::CertificationError(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "org.varlink.certification.CertificationError: '{}'",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
            Error_::VarlinkError_(e) => e.into(),
            Error_::IOError_(e) => e,
            Error_::JSONError_(e) => e.into(),
            Error_::UnknownError_(e) => io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "unknown varlink error: {}",
                    serde_json::to_string_pretty(&e).unwrap()
                ),
            ),
        }
    }
}
pub trait _CallEnd: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallEnd for varlink::Call<'a> {}

pub trait _CallStart: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallStart for varlink::Call<'a> {}

pub trait _CallTest01: _CallErr {
    fn reply(&mut self, bool: bool) -> io::Result<()> {
        self.reply_struct(Test01Reply_ { bool }.into())
    }
}

impl<'a> _CallTest01 for varlink::Call<'a> {}

pub trait _CallTest02: _CallErr {
    fn reply(&mut self, int: i64) -> io::Result<()> {
        self.reply_struct(Test02Reply_ { int }.into())
    }
}

impl<'a> _CallTest02 for varlink::Call<'a> {}

pub trait _CallTest03: _CallErr {
    fn reply(&mut self, float: f64) -> io::Result<()> {
        self.reply_struct(Test03Reply_ { float }.into())
    }
}

impl<'a> _CallTest03 for varlink::Call<'a> {}

pub trait _CallTest04: _CallErr {
    fn reply(&mut self, string: String) -> io::Result<()> {
        self.reply_struct(Test04Reply_ { string }.into())
    }
}

impl<'a> _CallTest04 for varlink::Call<'a> {}

pub trait _CallTest05: _CallErr {
    fn reply(&mut self, bool: bool, int: i64, float: f64, string: String) -> io::Result<()> {
        self.reply_struct(
            Test05Reply_ {
                bool,
                int,
                float,
                string,
            }.into(),
        )
    }
}

impl<'a> _CallTest05 for varlink::Call<'a> {}

pub trait _CallTest06: _CallErr {
    fn reply(&mut self, struct_: Test06Reply_struct) -> io::Result<()> {
        self.reply_struct(Test06Reply_ { struct_ }.into())
    }
}

impl<'a> _CallTest06 for varlink::Call<'a> {}

pub trait _CallTest07: _CallErr {
    fn reply(&mut self, map: varlink::StringHashMap<String>) -> io::Result<()> {
        self.reply_struct(Test07Reply_ { map }.into())
    }
}

impl<'a> _CallTest07 for varlink::Call<'a> {}

pub trait _CallTest08: _CallErr {
    fn reply(&mut self, set: varlink::StringHashSet) -> io::Result<()> {
        self.reply_struct(Test08Reply_ { set }.into())
    }
}

impl<'a> _CallTest08 for varlink::Call<'a> {}

pub trait _CallTest09: _CallErr {
    fn reply(&mut self, mytype: MyType) -> io::Result<()> {
        self.reply_struct(Test09Reply_ { mytype }.into())
    }
}

impl<'a> _CallTest09 for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn end(&self, call: &mut _CallEnd, mytype: MyType) -> io::Result<()>;
    fn start(&self, call: &mut _CallStart) -> io::Result<()>;
    fn test01(&self, call: &mut _CallTest01) -> io::Result<()>;
    fn test02(&self, call: &mut _CallTest02, bool: bool) -> io::Result<()>;
    fn test03(&self, call: &mut _CallTest03, int: i64) -> io::Result<()>;
    fn test04(&self, call: &mut _CallTest04, float: f64) -> io::Result<()>;
    fn test05(&self, call: &mut _CallTest05, string: String) -> io::Result<()>;
    fn test06(
        &self,
        call: &mut _CallTest06,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> io::Result<()>;
    fn test07(&self, call: &mut _CallTest07, struct_: Test07Args_struct) -> io::Result<()>;
    fn test08(&self, call: &mut _CallTest08, map: varlink::StringHashMap<String>)
        -> io::Result<()>;
    fn test09(&self, call: &mut _CallTest09, set: varlink::StringHashSet) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub trait VarlinkClientInterface {
    fn end(
        &mut self,
        mytype: MyType,
    ) -> io::Result<varlink::MethodCall<EndArgs_, EndReply_, Error_>>;
    fn start(&mut self) -> io::Result<varlink::MethodCall<StartArgs_, StartReply_, Error_>>;
    fn test01(&mut self) -> io::Result<varlink::MethodCall<Test01Args_, Test01Reply_, Error_>>;
    fn test02(
        &mut self,
        bool: bool,
    ) -> io::Result<varlink::MethodCall<Test02Args_, Test02Reply_, Error_>>;
    fn test03(
        &mut self,
        int: i64,
    ) -> io::Result<varlink::MethodCall<Test03Args_, Test03Reply_, Error_>>;
    fn test04(
        &mut self,
        float: f64,
    ) -> io::Result<varlink::MethodCall<Test04Args_, Test04Reply_, Error_>>;
    fn test05(
        &mut self,
        string: String,
    ) -> io::Result<varlink::MethodCall<Test05Args_, Test05Reply_, Error_>>;
    fn test06(
        &mut self,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> io::Result<varlink::MethodCall<Test06Args_, Test06Reply_, Error_>>;
    fn test07(
        &mut self,
        struct_: Test07Args_struct,
    ) -> io::Result<varlink::MethodCall<Test07Args_, Test07Reply_, Error_>>;
    fn test08(
        &mut self,
        map: varlink::StringHashMap<String>,
    ) -> io::Result<varlink::MethodCall<Test08Args_, Test08Reply_, Error_>>;
    fn test09(
        &mut self,
        set: varlink::StringHashSet,
    ) -> io::Result<varlink::MethodCall<Test09Args_, Test09Reply_, Error_>>;
}

pub struct VarlinkClient {
    connection: Arc<RwLock<varlink::Connection>>,
    more: bool,
}

impl VarlinkClient {
    pub fn new(connection: Arc<RwLock<varlink::Connection>>) -> Self {
        VarlinkClient {
            connection,
            more: false,
        }
    }
    pub fn more(&self) -> Self {
        VarlinkClient {
            connection: self.connection.clone(),
            more: true,
        }
    }
}

impl VarlinkClientInterface for VarlinkClient {
    fn end(
        &mut self,
        mytype: MyType,
    ) -> io::Result<varlink::MethodCall<EndArgs_, EndReply_, Error_>> {
        varlink::MethodCall::<EndArgs_, EndReply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.End".into(),
            EndArgs_ { mytype },
            self.more,
        )
    }
    fn start(&mut self) -> io::Result<varlink::MethodCall<StartArgs_, StartReply_, Error_>> {
        varlink::MethodCall::<StartArgs_, StartReply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Start".into(),
            StartArgs_ {},
            self.more,
        )
    }
    fn test01(&mut self) -> io::Result<varlink::MethodCall<Test01Args_, Test01Reply_, Error_>> {
        varlink::MethodCall::<Test01Args_, Test01Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test01".into(),
            Test01Args_ {},
            self.more,
        )
    }
    fn test02(
        &mut self,
        bool: bool,
    ) -> io::Result<varlink::MethodCall<Test02Args_, Test02Reply_, Error_>> {
        varlink::MethodCall::<Test02Args_, Test02Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test02".into(),
            Test02Args_ { bool },
            self.more,
        )
    }
    fn test03(
        &mut self,
        int: i64,
    ) -> io::Result<varlink::MethodCall<Test03Args_, Test03Reply_, Error_>> {
        varlink::MethodCall::<Test03Args_, Test03Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test03".into(),
            Test03Args_ { int },
            self.more,
        )
    }
    fn test04(
        &mut self,
        float: f64,
    ) -> io::Result<varlink::MethodCall<Test04Args_, Test04Reply_, Error_>> {
        varlink::MethodCall::<Test04Args_, Test04Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test04".into(),
            Test04Args_ { float },
            self.more,
        )
    }
    fn test05(
        &mut self,
        string: String,
    ) -> io::Result<varlink::MethodCall<Test05Args_, Test05Reply_, Error_>> {
        varlink::MethodCall::<Test05Args_, Test05Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test05".into(),
            Test05Args_ { string },
            self.more,
        )
    }
    fn test06(
        &mut self,
        bool: bool,
        int: i64,
        float: f64,
        string: String,
    ) -> io::Result<varlink::MethodCall<Test06Args_, Test06Reply_, Error_>> {
        varlink::MethodCall::<Test06Args_, Test06Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test06".into(),
            Test06Args_ {
                bool,
                int,
                float,
                string,
            },
            self.more,
        )
    }
    fn test07(
        &mut self,
        struct_: Test07Args_struct,
    ) -> io::Result<varlink::MethodCall<Test07Args_, Test07Reply_, Error_>> {
        varlink::MethodCall::<Test07Args_, Test07Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test07".into(),
            Test07Args_ { struct_ },
            self.more,
        )
    }
    fn test08(
        &mut self,
        map: varlink::StringHashMap<String>,
    ) -> io::Result<varlink::MethodCall<Test08Args_, Test08Reply_, Error_>> {
        varlink::MethodCall::<Test08Args_, Test08Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test08".into(),
            Test08Args_ { map },
            self.more,
        )
    }
    fn test09(
        &mut self,
        set: varlink::StringHashSet,
    ) -> io::Result<varlink::MethodCall<Test09Args_, Test09Reply_, Error_>> {
        varlink::MethodCall::<Test09Args_, Test09Reply_, Error_>::call(
            self.connection.clone(),
            "org.varlink.certification.Test09".into(),
            Test09Args_ { set },
            self.more,
        )
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"# Interface to test varlink implementations against.
# First you write a varlink client calling:
# Start, Test01, Test02, …, Testxx, End
# The return value of the previous call should be the argument of the next call.
# Then you test this client against well known servers like python or rust from
# https://github.com/varlink/
#
# Next you write a varlink server providing the same service as the well known ones.
# Now run your client against it and run well known clients like python or rust
# from https://github.com/varlink/ against your server. If all works out, then
# your new language bindings should be varlink certified.
interface org.varlink.certification

type Interface (
  foo: ?[]?[string](foo, bar, baz),
  anon: (foo: bool, bar: bool)
)

type MyType (
  object: object,
  enum: (one, two, three),
  struct: (first: int, second: string),
  array: []string,
  dictionary: [string]string,
  stringset: [string](),
  nullable: ?string,
  nullable_array_struct: ?[](first: int, second: string),
  interface: Interface
)

method Start() -> ()

method Test01() -> (bool: bool)

method Test02(bool: bool) -> (int: int)

method Test03(int: int) -> (float: float)

method Test04(float: float) -> (string: string)

method Test05(string: string) -> (
  bool: bool,
  int: int,
  float: float,
  string: string
)

method Test06(
  bool: bool,
  int: int,
  float: float,
  string: string
) -> (
  struct: (
    bool: bool,
    int: int,
    float: float,
    string: string
  )
)

method Test07(
  struct: (
    bool: bool,
    int: int,
    float: float,
    string: string
  )
) -> (map: [string]string)

method Test08(map: [string]string) -> (set: [string]())

method Test09(set: [string]()) -> (mytype: MyType)

method End(mytype: MyType) -> ()

error CertificationError (wants: string, got: string)
"#
    }

    fn get_name(&self) -> &'static str {
        "org.varlink.certification"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        match req.method.as_ref() {
            "org.varlink.certification.End" => {
                if let Some(args) = req.parameters.clone() {
                    let args: EndArgs_ = serde_json::from_value(args)?;
                    return self.inner.end(call as &mut _CallEnd, args.mytype);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Start" => {
                return self.inner.start(call as &mut _CallStart);
            }
            "org.varlink.certification.Test01" => {
                return self.inner.test01(call as &mut _CallTest01);
            }
            "org.varlink.certification.Test02" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test02Args_ = serde_json::from_value(args)?;
                    return self.inner.test02(call as &mut _CallTest02, args.bool);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test03" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test03Args_ = serde_json::from_value(args)?;
                    return self.inner.test03(call as &mut _CallTest03, args.int);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test04" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test04Args_ = serde_json::from_value(args)?;
                    return self.inner.test04(call as &mut _CallTest04, args.float);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test05" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test05Args_ = serde_json::from_value(args)?;
                    return self.inner.test05(call as &mut _CallTest05, args.string);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test06" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test06Args_ = serde_json::from_value(args)?;
                    return self.inner.test06(
                        call as &mut _CallTest06,
                        args.bool,
                        args.int,
                        args.float,
                        args.string,
                    );
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test07" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test07Args_ = serde_json::from_value(args)?;
                    return self.inner.test07(call as &mut _CallTest07, args.struct_);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test08" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test08Args_ = serde_json::from_value(args)?;
                    return self.inner.test08(call as &mut _CallTest08, args.map);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }
            "org.varlink.certification.Test09" => {
                if let Some(args) = req.parameters.clone() {
                    let args: Test09Args_ = serde_json::from_value(args)?;
                    return self.inner.test09(call as &mut _CallTest09, args.set);
                } else {
                    return call.reply_invalid_parameter("parameters".into());
                }
            }

            m => {
                return call.reply_method_not_found(String::from(m));
            }
        }
    }
}
