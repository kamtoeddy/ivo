import { OrderItem } from './orderItem';
import { StoreItemModel } from './storeItem';
import { CommonInheritanceTest } from './storeItem/common-tests';

CommonInheritanceTest(StoreItemModel, 'StoreItemModel');
CommonInheritanceTest(OrderItem, 'OrderItem');
