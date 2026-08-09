import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.required.onDelete', () => {
  it('should trigger onDelete handlers', async () => {
    let triggeredWith: string | undefined;
    let secondHandlerCalled = false;

    const Model = new Schema<{ required: string }>((b) =>
      b.field(
        b
          .required('required')
          .validate(() => ({ valid: true }))
          .onDelete([
            (data) => {
              triggeredWith = data.required;
            },
            async () => {
              secondHandlerCalled = true;
            },
          ]),
      ),
    ).getModel();

    await Model.delete({ required: 'required_string_value' }, {});

    expect(triggeredWith).toBe('required_string_value');
    expect(secondHandlerCalled).toBe(true);
  });
});
