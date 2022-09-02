export interface IOtherQuantity {
  name: string;
  quantity: number;
}

export interface IOtherMeasureUnit {
  coefficient: number;
  name: string;
}

export interface IStoreItem {
  c_At?: string;
  u_At?: string;
  _laxProp: string;
  _readOnlyLax1: string;
  _readOnlyLax2: string;
  _readOnlyNoInit?: any;
  _dependentReadOnly?: any;
  _sideEffectForDependentReadOnly?: any;
  id: string;
  name: string;
  price: number;
  quantityChangeCounter?: number;
  quantities?: IOtherQuantity[];
  quantity?: number;
  measureUnit: string;
  otherMeasureUnits?: IOtherMeasureUnit[];
}

export interface IStoreItemChild extends Omit<IStoreItem, "c_At" | "u_At"> {
  childID: string;
  createdAt?: string;
  updatedAt?: string;
}
