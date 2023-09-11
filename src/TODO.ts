import { Schema } from '.'

// TODO write tests to make sure this logic is respected

const data = {
  _id: 'FP4LK2H4LKV',
  accountStatus: 'active',
  name: 'Admin',
  role: 'admin',
  __typename: 'User'
}

// update = {
//   __typename: 'User',
//   _id: 'FP4LK2H4LKV',
//   accountStatus: 'active',
//   name: 'Admin',
//   role: 'admin'
// }

const value = { prop: data }

const Model = new Schema({ prop: { default: data } }).getModel()

Model.update(value, { prop: data }).then(({ data }) => {
  console.log(data)
})
