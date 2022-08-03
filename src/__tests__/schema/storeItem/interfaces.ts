export interface IOtherMeasureUnit {
  coefficient: number;
  name: string;
}

export interface IStoreItem {
  id: string;
  name: string;
  price: number;
  quantity: number;
  measureUnit: string;
  otherMeasureUnits: IOtherMeasureUnit[];
}
