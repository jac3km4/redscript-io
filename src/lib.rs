use byte::ctx::LittleEndian;

mod bundle;
mod definition;
mod index;
mod instr;
mod util;

const ENDIANESS: LittleEndian = byte::LE;

pub use bundle::{BundleReader, ScriptBundle};
pub use definition::{
    Class, ClassFlags, Definition, Enum, EnumMember, Field, FieldFlags, Function, FunctionBody,
    FunctionFlags, Local, LocalFlags, Parameter, ParameterFlags, Property, SourceFile,
    SourceReference, Type, TypeKind, Visibility,
};
pub use index::{
    CNameIndex, ClassIndex, EnumIndex, EnumValueIndex, FieldIndex, FunctionIndex, LocalIndex,
    ParameterIndex, ResourceIndex, SourceFileIndex, StringIndex, TweakDbIndex, TypeIndex,
};
pub use instr::{Breakpoint, Conditional, Instr, Jump, Offset, Profile, Switch, SwitchLabel};

#[cfg(not(feature = "shared"))]
pub type Str<'a> = hipstr::LocalHipStr<'a>;

#[cfg(feature = "shared")]
pub type Str<'a> = hipstr::HipStr<'a>;
