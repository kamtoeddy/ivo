import { describe, expect, it } from 'bun:test';
import { Schema } from '../../../src';

type Coordinates = { lat: number; lon: number };

function customize(s: string) {
  return `customized: ${s}`;
}

function validateCoordinates(c: unknown) {
  const coords = c as Coordinates;

  if (Number.isNaN(coords.lat) || Number.isNaN(coords.lon))
    return { valid: false, reason: 'InvalidNumber' };

  const errors: string[] = [];

  if (coords.lat < -90 || coords.lat > 90)
    errors.push('LatitudeOutOfRange: [-90, 90]');

  if (coords.lon < -180 || coords.lon > 180)
    errors.push('LongitudeOutOfRange: [-180, 180]');

  return errors.length
    ? { valid: false, reason: 'Out of range error', metadata: errors }
    : { valid: true };
}

describe('extras.errorSanitizer', () => {
  it('should respect custom error sanitizer', async () => {
    type Data = { coordinates: Coordinates };
    const PlaceModel = new Schema<
      Data,
      Data,
      {},
      string[],
      Record<string, string[]>
    >((b) => b.field(b.required('coordinates').validate(validateCoordinates)), {
      sanitizeError(payload) {
        const errors: Record<string, string[]> = {};

        for (const [field, err] of Object.entries(payload)) {
          const fieldErrors = [customize(err.reason)];

          if (err.metadata)
            for (const e of err.metadata) fieldErrors.push(customize(e));

          errors[field] = fieldErrors;
        }

        return errors;
      },
    }).getModel();

    let { error } = await PlaceModel.create(
      { coordinates: { lat: Number.NaN, lon: Number.NaN } },
      {},
    );

    expect(error?.coordinates).toEqual([customize('InvalidNumber')]);

    ({ error } = await PlaceModel.create(
      { coordinates: { lat: 400, lon: -200 } },
      {},
    ));

    expect(error?.coordinates).toEqual([
      customize('Out of range error'),
      customize('LatitudeOutOfRange: [-90, 90]'),
      customize('LongitudeOutOfRange: [-180, 180]'),
    ]);

    const data = { coordinates: { lat: 3, lon: 45.1 } };

    let { error: updateError } = await PlaceModel.update(
      data,
      { coordinates: { lat: Number.NaN, lon: Number.NaN } },
      {},
    );

    expect(updateError?.payload?.coordinates).toEqual([
      customize('InvalidNumber'),
    ]);

    ({ error: updateError } = await PlaceModel.update(
      data,
      { coordinates: { lat: -400, lon: 200 } },
      {},
    ));

    expect(updateError?.payload?.coordinates).toEqual([
      customize('Out of range error'),
      customize('LatitudeOutOfRange: [-90, 90]'),
      customize('LongitudeOutOfRange: [-180, 180]'),
    ]);

    const updatedCoords = {
      lat: data.coordinates.lat + 1.1,
      lon: data.coordinates.lon,
    };

    const { data: updates } = await PlaceModel.update(
      data,
      { coordinates: updatedCoords },
      {},
    );

    expect(updates).toEqual({ coordinates: updatedCoords });

    const updated = { coordinates: updatedCoords };

    ({ error: updateError } = await PlaceModel.update(
      updated,
      { coordinates: updated.coordinates },
      {},
    ));

    expect(updateError).toEqual({ isNothingToUpdate: true, payload: null });
  });
});
