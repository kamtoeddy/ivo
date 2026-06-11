use crate::{schema::options::base::SchemaOptions, IvoErrorTool, IvoSchemaStruct};

pub trait SchemaInternals<
    I: IvoSchemaStruct,
    O: IvoSchemaStruct,
    CtxOptions,
    ErrorTool: IvoErrorTool,
>
{
    // Schema {
    //     field_configs: InternalFieldConfigs<I, O, CtxOptions, ErrorTool>,
    //     options: SchemaOptions<I, O, CtxOptions, ErrorTool>
    fn options(&self) -> &SchemaOptions<I, O, CtxOptions, ErrorTool>;
}
