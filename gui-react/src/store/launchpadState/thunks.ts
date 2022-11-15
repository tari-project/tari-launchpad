import { createAsyncThunk } from '@reduxjs/toolkit'
import { emit, listen } from '@tauri-apps/api/event'
import { LaunchPad, LaunchpadSession } from './types'
import { setLaunchpadState } from './index'
import { RootState } from '../index'

export const getLaunchPadState = createAsyncThunk(
  'app/getLaunchPadState',
  async (_, thunkApi) => {
    try {
      const unlisten = await listen(
        'tari://reactions',
        (event: { payload: LaunchPad }) => {
          if (event.payload.State) {
            thunkApi.dispatch(setLaunchpadState(event.payload.State))
          }
        },
      )

      return { unlisten }
    } catch (error) {
      return thunkApi.rejectWithValue(error)
    }
  },
)

export const changeSession = createAsyncThunk<
  void,
  { sessionItem?: Partial<LaunchpadSession> },
  { state: RootState }
>('app/changeSession', async ({ sessionItem }, thunkApi) => {
  try {
    const rootState = thunkApi.getState()

    const currentSession =
      rootState.launchpadState.launchpadState?.config?.session

    const sessionUpdate: Partial<LaunchpadSession> = {
      ...currentSession,
      ...sessionItem,
    }

    //TODO ask about shared_grafana_volume_active

    await emit('tari://actions', {
      Action: {
        ChangeSession: sessionUpdate,
      },
    })
  } catch (error) {
    return thunkApi.rejectWithValue(error)
  }
})

export const connect = createAsyncThunk('app/connect', async (_, thunkApi) => {
  try {
    await emit('tari://actions', { Action: 'Connect' })
  } catch (error) {
    return thunkApi.rejectWithValue(error)
  }
})
