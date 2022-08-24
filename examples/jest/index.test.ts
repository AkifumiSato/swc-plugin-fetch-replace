import { localFetcher } from './index'

type GlobalThis = {
  replace_fetch: () => Promise<{
    json: () => Promise<Record<string, string>>
  }>
}

declare var globalThis: GlobalThis;

test('localFetcher', async () => {
  const body = {
    name: 'akfm',
  }
  globalThis.replace_fetch = async () => ({
    json: async () => (body)
  })
  expect(await localFetcher()).toBe(body);
})
