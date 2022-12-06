import { IGetRegistryListArgs } from './src/types/registry'
import {
  mockAllListings,
  mockDappListings,
  mockProjectDataResponse,
  mockRegistryChains,
  mockWalletListings,
} from './test/__mocks__/fixtures'

// Mock global `fetch`
// @ts-expect-error - ignore that this is an incomplete mock of `fetch`.
global.fetch = jest.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve({}),
  }),
)

// @ts-expect-error - add the internal API auth token to global namespace as required by auth handler.
global.INTERNAL_API_AUTH_TOKEN = 'VALID_AUTH_TOKEN'

// Mock SupabaseService methods
jest.mock('./src/services/SupabaseService', () => ({
  getRegistryChains: jest.fn(() => Promise.resolve(mockRegistryChains)),
  getImageId: jest.fn((publicId) =>
    Promise.resolve(`mock-image-id-for-${publicId}`),
  ),
  isValidProject: jest.fn((projectId) =>
    Promise.resolve(true),
  ),
  getRegistryListings: jest.fn(({ appType }: IGetRegistryListArgs) => {
    if (!appType || appType === 'all') {
      return Promise.resolve({
        count: mockAllListings.count,
        data: mockAllListings.listings,
      })
    } else if (appType === 'wallets') {
      return Promise.resolve({
        count: mockWalletListings.count,
        data: mockWalletListings.listings,
      })
    } else if (appType === 'dapps') {
      return Promise.resolve({
        count: mockDappListings.count,
        data: mockDappListings.listings,
      })
    }
  }),

  getProjectByKey: jest.fn((projectIdKey: string) => {
    return Promise.resolve(mockProjectDataResponse[projectIdKey] || null)
  })
}))
