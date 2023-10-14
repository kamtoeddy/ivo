import { Schema } from '../../../dist';
import { makeFx } from './_utils';

import { Test_BasicDefinitions } from './definitions/basic';
import { Test_ConstantProperties } from './definitions/constant-properties';
import { Test_DependentProperties } from './definitions/dependent-properties';
import { Test_LaxProperties } from './definitions/lax-properties';
import { Test_ReadonlyProperties } from './definitions/readonly-properties';
import { Test_RequiredProperties } from './definitions/required-properties';
import { Test_VirtualProperties } from './definitions/virtual-properties';
import { Test_LifeCycleHandlers } from './definitions/life-cycle-handlers';
import { Test_ShouldInitRule } from './definitions/should-init-rule';
import { Test_ShouldUpdateRule } from './definitions/should-update-rule';
import { Test_SchemaErrors } from './options/errors';
import { Test_SchemaOptionFormat } from './options/format';
import { Test_SchemaTimestampOption } from './options/timestamps';
import { Test_Validators } from './validators';
import { Test_SchemaOnSuccess } from './options/on-success';
import { Test_SchemaOnDelete } from './options/on-delete';
import { Test_SchemaShouldUpdateOption } from './options/should-update';
import { Test_SchemaSetMissingDefaultsOnUpdateOption } from './options/set-missing-defaults-on-update';
import { Test_SchemaEqualityDepth } from './options/equality-depth';
import { describe, test } from 'vitest';
import { Test_ExtendedSchemas } from './definitions/extended-schemas';

const fx = makeFx(Schema);

describe('Dist', () => {
  test.todo('tests');
});

Test_BasicDefinitions({ fx, Schema });

Test_Validators({ Schema });

Test_ConstantProperties({ Schema, fx });
Test_DependentProperties({ Schema, fx });
Test_ExtendedSchemas({ Schema });
Test_LaxProperties({ Schema, fx });
Test_ReadonlyProperties({ Schema, fx });
Test_RequiredProperties({ Schema, fx });
Test_VirtualProperties({ Schema, fx });

Test_LifeCycleHandlers({ Schema, fx });
Test_ShouldInitRule({ Schema, fx });
Test_ShouldUpdateRule({ Schema, fx });

Test_SchemaErrors({ Schema, fx });
Test_SchemaEqualityDepth({ Schema, fx });
Test_SchemaOnDelete({ Schema, fx });
Test_SchemaOnSuccess({ Schema, fx });
Test_SchemaOptionFormat({ fx });
Test_SchemaSetMissingDefaultsOnUpdateOption({ Schema, fx });
Test_SchemaShouldUpdateOption({ Schema, fx });
Test_SchemaTimestampOption({ Schema, fx });
