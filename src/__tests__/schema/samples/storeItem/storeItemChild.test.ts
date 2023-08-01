import { CommonInheritanceTest, commonTestData } from './common-tests'
import { StoreItemChild } from './storeItemChild'

const testData = { ...commonTestData, childID: '1' }

CommonInheritanceTest('StoreItemChild', StoreItemChild, testData)

describe('Testing non-inherited properties for StoreItemChild', () => {
  let item: any

  beforeAll(async () => {
    item = (await StoreItemChild.create(testData)).data
  })

  // creation
  it('should have the correct properties at creation', () => {
    expect(item).toMatchObject({ childID: '1' })

    expect(item).toHaveProperty('createdAt')
    expect(item).toHaveProperty('updatedAt')
  })

  // updates
  it('should have the correct properties after updates', async () => {
    const { data: update } = await StoreItemChild.update(item, {
      childID: '12',
      name: 'Guiness '
    } as any)

    expect(update!.childID).toBe(undefined)
    expect(update).toMatchObject({ name: 'Guiness' })
    expect(update).toHaveProperty('updatedAt')
  })
})
