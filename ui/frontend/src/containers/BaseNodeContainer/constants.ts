export const networks = ['nextnet', 'testnet'];

function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1);
}
export const networkOptions = networks.map((network) => ({
  label: capitalizeFirstLetter(network),
  value: network,
  key: network,
}));
