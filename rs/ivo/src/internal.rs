use crate::IvoSchemaStruct;

pub(crate) trait InitializableIvoContext<I: IvoSchemaStruct, O: IvoSchemaStruct> {
    fn for_new(input: I::Partial, input_values: I::Partial, values: O::Partial) -> Self;
    fn for_update(
        changes: O::Partial,
        input: I::Partial,
        input_values: I::Partial,
        previous_values: O,
        values: O,
    ) -> Self;
}
