import axios from 'axios'

declare let process: {
  env: {
    JEST_ENV: string
  }
}

const BASE_URLS = new Map<string, string>([
  ['staging', 'https://staging.echo.walletconnect.com'],
  ['dev', 'http://localhost:3000'],
])

const BASE_URL = BASE_URLS.get(process.env.JEST_ENV)!

describe('Echo Server', () => {
  describe('Health', () => {
    const url = `${BASE_URL}/health`

    it('is healthy', async () => {
      const { status } = await axios.get(`${url}`)

      expect(status).toBe(200)
    })
  })
})
