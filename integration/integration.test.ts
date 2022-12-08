import axios from 'axios'

declare let process: {
  env: {
    JEST_ENV: string
  }
}

const BASE_URLS = new Map<string, string>([
  ['prod', 'https://echo.walletconnect.com'],
  ['staging', 'https://staging.echo.walletconnect.com'],
  ['dev', 'http://localhost:3000'],
])

const TEST_TENANT = 'f1e0fcb9-75f8-49b5-a05b-00c35ac8418f'

const BASE_URL = BASE_URLS.get(process.env.JEST_ENV)

describe('Echo Server', () => {
  describe('Health', () => {
    const url = `${BASE_URL}/health`

    it('is healthy', async () => {
      const { status } = await axios.get(`${url}`)

      expect(status).toBe(200)
    })
  })
  describe('Client Registration', () => {
    const url = `${BASE_URL}/${TEST_TENANT}/clients`

    it('registers a client', async () => {
      const { status, data } = await axios.post(
        `${url}`,
        {
          client_id: Math.random().toString(36).substr(2, 5),
          type: 'apns',
          token: Math.random().toString(36).substr(2, 5),
        },
        {
          headers: {
            'content-type': 'application/json',
          },
        },
      )

      expect(status).toBe(200)
    })
  })
})
