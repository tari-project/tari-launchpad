import { Container, ContainerId } from '../../../../store/containers/types'

type ContainerDto = {
  id: ContainerId | undefined
  container: Container
  // cpu: number
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  error?: any
  // memory: number
  pending: boolean
  running: boolean
}
// & Pick<DockerImage, 'imageName' | 'displayName'>

export type ContainersProps = {
  containers: ContainerDto[]
  updateSession: (container: Container, stop?: boolean) => void
}
