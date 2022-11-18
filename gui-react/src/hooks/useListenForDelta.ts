import { useEffect, useState } from 'react'
import {
  setLaunchpadDelta,
  updateStateFromDelta,
} from '../store/launchpadState'
import { LaunchPad, LaunchpadDelta } from '../store/launchpadState/types'
import { listen } from '@tauri-apps/api/event'
import { AppDispatch } from '../store'
import { addStats } from '../store/containers/thunks'

export const useListenForDelta = ({ dispatch }: { dispatch: AppDispatch }) => {
  const [delta, setDelta] = useState<LaunchpadDelta>()
  useEffect(() => {
    const unlisten = listen(
      'tari://reactions',
      (event: { payload: LaunchPad }) => {
        if (event.payload) {
          const { Delta, State } = event.payload
          setDelta(Delta)

          console.log(`Delta: ${JSON.stringify(Delta)}`)
          console.log(`State: ${JSON.stringify(State)}`)

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
