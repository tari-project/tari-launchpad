import { useMemo, useState } from 'react'

import { useAppDispatch, useAppSelector } from '../../../../store/hooks'
import { Container } from '../../../../store/containers/types'
import Alert from '../../../../components/Alert'

import Containers from './Containers'
import { selectContainerStates } from '../../../../store/launchpadState/selectors'
import { changeSession } from '../../../../store/launchpadState'
import {
  Containers as ContainerNameKey,
  StatsData,
  TaskStatus,
} from '../../../../store/launchpadState/types'
import { selectContainersStatusesWithStats } from '../../../../store/containers/selectors'

const ContainersContainer = () => {
  const [error, setError] = useState('')

  const dispatch = useAppDispatch()
  const containerStates = useAppSelector(selectContainerStates)

  const containerStatuses = useAppSelector(selectContainersStatusesWithStats)

  // console.log(`containerStatuses: ${JSON.stringify(containerStatuses)}`)

  const containers = useMemo(
    () =>
      containerStates.map(({ id, task_state }) => ({
        id: id,
        container:
          Container[
            id == 'Base Node'
              ? ('BaseNode' as keyof typeof Container)
              : (ContainerNameKey[
                  id as keyof typeof ContainerNameKey
                ] as keyof typeof Container)
          ],
        pending: task_state?.status === TaskStatus.Pending,
        running: task_state?.status === TaskStatus.Active,
        progress:
          task_state?.status === TaskStatus.Progress ? task_state?.status : {},
        stats:
          task_state?.stats[task_state?.stats?.length - 1] ||
          ({ cpu_usage: 0, mem_usage: 0 } as StatsData),
      })),
    [containerStates],
  )

  const updateSession = async (container: Container, stop?: boolean) => {
    try {
      const containerField = `${container}_active`
      await dispatch(
        changeSession({ sessionItem: { [containerField]: !stop } }),
      ).unwrap()
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (e: any) {
      setError(e.toString())
    }
  }

  return (
    <>
      <Containers containers={containers} updateSession={updateSession} />
      <Alert
        title='Error'
        open={Boolean(error)}
        onClose={() => setError('')}
        content={error}
      />
    </>
  )
}

export default ContainersContainer
