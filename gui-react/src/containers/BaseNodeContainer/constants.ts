export const networks = ['esmeralda', 'dibbler', 'testnet']

export const networkOptions = networks.map(network => ({
  label: network,
  value: network,
  key: network,
}))
