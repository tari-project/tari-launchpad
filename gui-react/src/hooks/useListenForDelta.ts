import { useEffect, useState } from 'react'
import {
  setLaunchpadDelta,
  updateStateFromDelta,
} from '../store/launchpadState'
import { LaunchPad, LaunchpadDelta } from '../store/launchpadState/types'
import { listen } from '@tauri-apps/api/event'
import { AppDispatch } from '../store'

export const useListenForDelta = ({ dispatch }: { dispatch: AppDispatch }) => {
  const [delta, setDelta] = useState<LaunchpadDelta>()
  useEffect(() => {
    const unlisten = listen(
      'tari://reactions',
      (event: { payload: LaunchPad }) => {
        if (event.payload) {
          const { Delta } = event.payload
          setDelta(Delta)

          dispatch(setLaunchpadDelta(Delta))
          dispatch(updateStateFromDelta(Delta))
        }
      },
    )
    return () => {
      unlisten.then(d => d())
    }
  }, [])
  return { delta }
}
