import { describe, expect, it } from 'bun:test';

import { Schema } from '../src';

const validator = (value: unknown) => ({
  valid: true as const,
  validated: value,
});

describe('regressions', () => {
  it("should call a required field's ignoreUpdate resolver once per update", async () => {
    let calls = 0;

    const Model = new Schema<{ name: string; bio: string }>((s) => {
      s.field(
        s
          .required('name')
          .validate(validator as never)
          .ignoreUpdate(() => {
            calls++;
            return false;
          }),
      );
      s.field(s.lax('bio', ''));
      return s;
    }).getModel();

    const { data } = await Model.create({ name: 'a' }, {});
    calls = 0;

    const { data: updates } = await Model.update(data!, { name: 'b' }, {});

    expect(calls).toBe(1);
    expect(updates).toEqual({ name: 'b' });
  });

  it("should not freeze the caller's data and options on delete", async () => {
    const received: { data?: unknown; options?: unknown } = {};

    const Model = new Schema<
      { name: string },
      { name: string },
      { tag: string }
    >(
      (s) => {
        s.field(s.required('name').validate(validator as never));
        return s;
      },
      {
        onDelete: (data: unknown, options: unknown) => {
          received.data = data;
          received.options = options;
        },
      } as never,
    ).getModel();

    const data = { name: 'a' };
    const options = { tag: 't' };
    await Model.delete(data, options);

    expect(Object.isFrozen(data)).toBe(false);
    expect(Object.isFrozen(options)).toBe(false);

    // handlers still get frozen copies of the same values
    expect(received.data).toEqual(data);
    expect(Object.isFrozen(received.data)).toBe(true);
    expect(received.options).toEqual(options);
    expect(Object.isFrozen(received.options)).toBe(true);
  });
});
