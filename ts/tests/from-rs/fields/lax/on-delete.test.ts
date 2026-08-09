import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../../src';

describe('fields.lax.onDelete', () => {
  it('should trigger onDelete handlers', async () => {
    let secondHandlerCalled = false;
    let triggeredWith: string | undefined;

    const Model = new Schema<{ lax: string }>((b) =>
      b.field(
        b
          .lax('lax', 'default_value')
          .validate((v) => ({ valid: true, validated: v as string }))
          .onDelete([
            async () => {
              secondHandlerCalled = true;
            },
            (data) => {
              triggeredWith = data.lax;
            },
          ]),
      ),
    ).getModel();

    await Model.delete({ lax: 'lax_string_value' }, {});

    expect(secondHandlerCalled).toBe(true);
    expect(triggeredWith).toBe('lax_string_value');
  });
});
