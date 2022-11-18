import {
  Containers,
  ContainerTaskState,
  LaunchPad,
  LaunchpadDelta,
  LaunchpadState,
  TaskDelta,
} from './types'
import { createAction, createSlice } from '@reduxjs/toolkit'

const initialContainers = Object.assign({}, ...Object.values(Containers))
export const launchpadStateInitialState: LaunchPad = {
  State: {
    config: {
      session: {},
      settings: {},
    },
    containers: initialContainers,
    wallet: { active: false, transactions: [] },
  },
  Delta: {},
  taskDeltas: [],
  launchpadState: {
    config: { session: {} },
    containers: [],
    wallet: { active: false, transactions: [] },
  },
}

export const setLaunchpadDelta =
  createAction<LaunchpadDelta>('setLaunchpadDelta')

export const updateStateFromDelta = createAction<LaunchpadDelta>(
  'updateStateFromDelta',
)

const launchpadStateSlice = createSlice({
  name: 'launchpadState',
  initialState: launchpadStateInitialState,
  reducers: {
    setLaunchpadState(state, { payload }: { payload: LaunchpadState }) {
      const containers = payload?.containers
      const containerStates: ContainerTaskState[] = Object.values(
        Containers,
      ).map(c => ({
        id: c,
        task_state: containers ? containers[c] : undefined,
      }))

      state.launchpadState = { ...payload, containers: containerStates }
      state.State = payload
    },
  },
  extraReducers: builder => {
    builder.addCase(setLaunchpadDelta, (state, action) => {
      const deltaArray: TaskDelta[] = []
      const newDelta = action.payload?.TaskDelta

      if (deltaArray && newDelta) {
        const existingTaskId = state.taskDeltas.find(d => d.id === newDelta.id)

        if (existingTaskId && existingTaskId !== -1) {
          state.taskDeltas[deltaArray.indexOf(existingTaskId)] = newDelta
        } else {
          deltaArray.push(newDelta)
        }
      }

      state.taskDeltas = state.taskDeltas
        ? [...state.taskDeltas, ...deltaArray]
        : deltaArray
    })
    builder.addCase(updateStateFromDelta, (state, action) => {
      if (action.payload?.UpdateConfig) {
        state.launchpadState.config = {
          ...state.launchpadState.config,
          ...action.payload.UpdateConfig,
        }
      }
      if (action.payload?.WalletDelta) {
        state.launchpadState.wallet = action.payload.WalletDelta
      }
      if (
        action.payload?.TaskDelta &&
        action.payload.TaskDelta.id &&
        action.payload.TaskDelta?.delta?.UpdateStatus
      ) {
        const containers = state.launchpadState.containers
        const c = containers.findIndex(
          c => c.id === action.payload.TaskDelta?.id,
        )

        containers[c] = {
          id: action.payload.TaskDelta.id,
          task_state: {
            status: action.payload.TaskDelta.delta.UpdateStatus,
            permanent:
              state.State.containers[action.payload.TaskDelta.id]?.permanent ||
              false,
          },
        }
        state.launchpadState.containers = containers
      }
    })
  },
})

export const { setLaunchpadState } = launchpadStateSlice.actions
export { connect, changeSession, getLaunchPadState } from './thunks'
const reducer = launchpadStateSlice.reducer
export default reducer
