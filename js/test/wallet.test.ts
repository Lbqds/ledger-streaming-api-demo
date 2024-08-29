import SpeculosTransport from '@ledgerhq/hw-transport-node-speculos'
import { AlephiumApp } from '../src/ledger-app'
import { randomBytes } from 'crypto'

describe('ledger wallet', () => {
  it('should test streaming api', async () => {
    const transport = await SpeculosTransport.open({ apduPort: 9999 })
    const app = new AlephiumApp(transport)
    const bytes = randomBytes(255 * 2 + 128)
    await app.sendData(bytes)
  }, 120000)
})
