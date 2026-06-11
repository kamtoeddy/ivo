use crate::IvoSchemaStruct;

pub(crate) trait InternalIvoContextMethods<I: IvoSchemaStruct, O: IvoSchemaStruct> {
    fn new_create_ctx(input: I::Partial, input_values: I::Partial, values: O::Partial) -> Self;

    fn new_update_ctx(
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
    ) -> Self;
}
