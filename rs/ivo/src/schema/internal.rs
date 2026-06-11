use crate::{schema::options::base::SchemaOptions, IvoErrorTool, IvoSchemaStruct};

pub trait SchemaInternals<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    fn options(&self) -> &SchemaOptions<I, O, CtxOptions, ErrorTool>;
}
