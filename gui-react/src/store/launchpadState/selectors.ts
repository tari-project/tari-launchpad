import { RootState } from '../index'

export const selectContainerStates = (rs: RootState) => {
  return rs?.launchpadState?.launchpadState?.containers
}
