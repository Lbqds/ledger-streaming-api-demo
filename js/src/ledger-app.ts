import Transport, { StatusCodes } from '@ledgerhq/hw-transport'

export const CLA = 0x80
export const INS = 0x00
const MAX_PAYLOAD_SIZE = 255

export class AlephiumApp {
  readonly transport: Transport

  constructor(transport: Transport) {
    this.transport = transport
  }

  async close(): Promise<void> {
    await this.transport.close()
  }

  async sendData(bytes: Buffer): Promise<void> {
    if (bytes.length <= 255) throw Error(`Invalid bytes length`)
    let index = 0
    while (index < bytes.length) {
      const frameLength = Math.min(bytes.length - index, MAX_PAYLOAD_SIZE)
      const frameData = bytes.slice(index, index + frameLength)
      await this.transport.send(CLA, INS, 0, 0, frameData, [StatusCodes.OK])
      index += frameLength
    }
    return
  }
}
