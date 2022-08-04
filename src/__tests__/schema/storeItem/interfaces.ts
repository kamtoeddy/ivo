export interface IOtherQuantity {
  name: string;
  quantity: number;
}

export interface IOtherMeasureUnit {
  coefficient: number;
  name: string;
}

export interface IStoreItem {
  id: string;
  name: string;
  price: number;
  quantityChangeCounter?: number;
  quantities?: IOtherQuantity[];
  quantity?: number;
  measureUnit: string;
  otherMeasureUnits?: IOtherMeasureUnit[];
}
