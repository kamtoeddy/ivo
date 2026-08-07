import { describe, expect, it } from "bun:test";
import { Schema } from "../src";

/**
 * Stress & pathological inputs tests
 *
 * These tests are intended to ensure ivo does not crash or hang when presented
 * with large or pathological input shapes. They do not assert specific business
 * validation semantics beyond "operation completes without throwing" and that
 * returned shapes exist or have expected sizes where reasonable.
 *
 * NOTE: These are stress-style tests. If your CI environment is constrained,
 * you may reduce the sizes below.
 */

describe("Stress tests — pathological input shapes", () => {
  it("handles very deep nested objects without throwing", async () => {
    // build a deep nested object (depth configurable)
    const makeDeep = (depth: number) => {
      const root: Record<string, any> = {};
      let cur = root;
      for (let i = 0; i < depth; i++) {
        cur.child = {};
        cur = cur.child;
      }
      return root;
    };

    const depth = 200; // adjust if CI can't handle this depth
    const deepObj = makeDeep(depth);

    const Model = new Schema<{ payload: {} }>((f) =>
      f.field(
        f
          .lax("payload", {})
          .validate((v) =>
            typeof v === "object" && v !== null
              ? { valid: true, validated: v }
              : { valid: false, reasons: ["Expected object"] },
          ),
      ),
    ).getModel();

    const { data, error } = await Model.create({ payload: deepObj }, {});
    expect(error).toBeNull();
    expect(data).not.toBeNull();
  });

  it("handles very large arrays without throwing and preserves length", async () => {
    const largeCount = 20_000; // moderate large array for CI; increase for local stress runs
    const bigArray = new Array(largeCount).fill("x");

    const Model = new Schema<{ payload: string[] }>((f) =>
      f.field(
        f
          .lax("payload", [])
          .validate((v) =>
            Array.isArray(v)
              ? { valid: true, validated: v }
              : { valid: false, reasons: ["Expected array"] },
          ),
      ),
    ).getModel();

    const { data, error } = await Model.create({ payload: bigArray }, {});
    expect(error).toBeNull();
    expect(data).toBeDefined();
    expect(Array.isArray(data!.payload)).toBeTruthy();
    expect(data!.payload.length).toBe(largeCount);
  });

  it("handles very long strings without throwing and returns validated value", async () => {
    const longLen = 20_000;
    // const longLen = 2_147_483_647; // 200k chars (reduce for restrictive CI)
    const longStr = "a".repeat(longLen);

    const Model = new Schema<{ text: string }>((f) =>
      f.field(
        f
          .lax("text", "")
          .validate((v) =>
            typeof v === "string"
              ? { valid: true, validated: v }
              : { valid: false, reasons: ["Expected string"] },
          ),
      ),
    ).getModel();

    const { data, error } = await Model.create({ text: longStr }, {});
    expect(error).toBeNull();
    expect(data).toBeDefined();
    expect(typeof data!.text).toBe("string");
    expect((data!.text as string).length).toBe(longLen);
  });

  it("handles circular references without crashing (structuredClone-capable environments)", async () => {
    const circular: any = { name: "root" };
    circular.self = circular; // circular reference

    const Model = new Schema<{ payload: {} }>((f) =>
      f.field(
        f
          .lax("payload", {})
          .validate((v) =>
            typeof v === "object" && v !== null
              ? { valid: true, validated: v }
              : { valid: false, reasons: ["Expected object"] },
          ),
      ),
    ).getModel();

    // We assert the operation completes (no uncaught synchronous exception).
    // Some internal code may use structuredClone (which supports circular refs)
    // or fall back to JSON methods; the latter will throw for circular refs.
    // The test will therefore highlight environments or code paths that don't
    // safely handle circular inputs.
    const { data, error } = await Model.create({ payload: circular }, {});
    expect(error).toBeNull();
    expect(data).not.toBeNull();
  });
});
