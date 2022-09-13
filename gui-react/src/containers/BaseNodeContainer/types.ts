export type Network = 'esmeralda' | 'dibbler' | 'testnet'

export interface BaseNodeProps {
  startNode: () => void
  stopNode: () => void
  tariNetwork: Network
  setTariNetwork: (selected: Network) => void
  running: boolean
  pending: boolean
}
