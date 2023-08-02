export { Merge, RealType }

type TypeFromPromise<T> = T extends Promise<infer I> ? I : T

type RealType_<T> = T extends (...args: any) => infer I ? I : T

type RealType<T> = {
  [K in keyof T]: TypeFromPromise<Exclude<T[K], Function> | RealType_<T[K]>>
} & {}

// type Person = {
//   id?: string
//   age: number
//   dob: number
//   placeOfBirth: null | number
//   name: string
// }

// type User = {
//   id: number
//   username: string
//   placeOfBirth: null | 'lol' | 'yooo'
//   getFullname:()=>"lol"
// }

// type Human = Merge<Person, User>

type Merge<I, O> = {
  [K in keyof I | keyof O]: Exclude<
    K extends keyof I
      ? K extends keyof O
        ? I[K] | O[K]
        : I[K]
      : K extends keyof O
      ? O[K]
      : never,
    undefined
  >
}
