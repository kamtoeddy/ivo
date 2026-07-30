export interface IOtherQuantity {
  name: string;
  quantity: number;
}

export interface IOtherMeasureUnit {
  coefficient: number;
  name: string;
}

export interface StoreItemInput {
  _laxProp: string;
  _quantity: number;
  _readOnlyLax1: string;
  _readOnlyLax2: string;
  _readOnlyNoInit: any;
  _virtualForDependentReadOnly?: any;
  id: string;
  name: string;
  price: number;
  quantities: IOtherQuantity[] | number;
  measureUnit: string;
  otherMeasureUnits: IOtherMeasureUnit[];
}

export interface StoreItem {
  _dependentReadOnly: number;
  _laxProp: string;
  _readOnlyLax1: string;
  _readOnlyLax2: string;
  _readOnlyNoInit: any;
  c_At: string;
  id: string;
  name: string;
  price: number;
  quantityChangeCounter: number;
  quantity: number;
  measureUnit: string;
  otherMeasureUnits: IOtherMeasureUnit[];
  u_At: string;
}

export interface IStoreItemChild extends StoreItemInput {
  childID: string;
}
