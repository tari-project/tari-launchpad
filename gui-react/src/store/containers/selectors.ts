import { createSelector } from '@reduxjs/toolkit'

import { RootState } from '../'
import { ContainerName } from '../../types/general'
import { selectDockerImages, selectRecipe } from '../dockerImages/selectors'

import {
  ContainerStatusDto,
  Container,
  ContainerStatusDtoWithStats,
} from './types'
import { Containers, TaskStatus } from '../launchpadState/types'

export const selectState = (rootState: RootState) => rootState.containers

export const selectContainer = (c: ContainerName) => (r: RootState) => {
  const indexOfC = Object.values(Container).indexOf(c as unknown as Container)
  const containerKey = Object.keys(Container)[indexOfC]

  const containers = r?.launchpadState?.launchpadState?.containers.filter(
    value => value.id == Containers[containerKey as keyof typeof Containers],
  )
  containers.sort((a, b) => b.timestamp - a.timestamp)
  const { id, task_state } = containers[0] || []

  return { containerId: id, containerStatus: task_state }
}

export const selectContainerError = (c: ContainerName) => (r: RootState) => {
  return r.containers.errors[c]
}

const selectContainerStats = (containerId: string) => (r: RootState) =>
  r.launchpadState.launchpadState?.containers?.find(c => c?.id === containerId)
    ?.task_state?.stats

type ContainerStatusSelector = (
  c: ContainerName,
) => (r: RootState) => ContainerStatusDto
export const selectContainerStatus: ContainerStatusSelector =
  containerName => rootState => {
    const { containerId, containerStatus } =
      selectContainer(containerName)(rootState)

    const pending =
      rootState.containers.pending.includes(containerName) ||
      rootState.containers.pending.includes(containerId)

    const typeError = rootState.containers.errors[containerName]

    if (!containerId) {
      return {
        id: '',
        containerName,
        error: typeError,
        running: false,
        pending,
      }
    }

    const { ...containerStatusWithoutName } = containerStatus
    return {
      ...containerStatusWithoutName,
      id: containerId,
      pending:
        containerStatus?.status === TaskStatus.Pending ||
        (containerStatus?.status !== TaskStatus.Active &&
          containerStatus?.status !== TaskStatus.Inactive),
      running: containerStatus?.status === TaskStatus.Active,
      containerName,
      error: containerStatus?.error || typeError,
    }
  }

type ContainerStatusSelectorWithStats = (
  c: ContainerName,
) => (r: RootState) => ContainerStatusDtoWithStats
export const selectContainerStatusWithStats: ContainerStatusSelectorWithStats =
  containerName => rootState => {
    const container = selectContainerStatus(containerName)(rootState)

    if (!container.id) {
      return {
        ...container,
        stats: [
          {
            cpu_usage: 0,
            mem_limit: 0,
            mem_usage: 0,
            // network: {
            //   download: 0,
            //   upload: 0,
            // },
            timestamp: '',
            // unsubscribe: () => undefined,
          },
        ],
      }
    }

    const containerStats = selectContainerStats(container.id)(rootState)

    return {
      ...container,
      stats: containerStats || [],
    }
  }

export const selectRunningContainers = (rootState: RootState): Container[] =>
  Object.entries(rootState.launchpadState.launchpadState.containers)
    .map(([, containerStatus]) =>
      selectContainerStatus(containerStatus.id as Containers)(rootState),
    )
    .filter(status => status.running)
    .map(status => rootState.containers.containers[status.id].name as Container)

export const selectContainersStatusesWithStats = createSelector(
  selectDockerImages,
  (rootState: RootState) => rootState,
  (dockerImages, rootState) =>
    dockerImages.map(dockerImage => ({
      ...dockerImage,
      container: dockerImage.containerName,
      status: selectContainerStatusWithStats(dockerImage.containerName)(
        rootState,
      ),
    })),
)

const selectContainerStatusesByRecipe =
  (containerName: ContainerName) => (rootState: RootState) => {
    const recipe = selectRecipe(containerName)(rootState)
    return recipe.map(containerType =>
      selectContainerStatus(containerType)(rootState),
    )
  }

export const selectRecipeRunning = (containerName: ContainerName) =>
  createSelector(
    selectContainerStatusesByRecipe(containerName),
    containers =>
      containers.every(container => container.running) ||
      containers.some(container => container.running && container.pending),
  )

export const selectRecipePending = (containerName: ContainerName) =>
  createSelector(selectContainerStatusesByRecipe(containerName), containers =>
    containers.some(container => container.pending),
  )

export const selectAllContainerEventsChannels = (rootState: RootState) =>
  Object.values(rootState.containers.containers)
    .map(container => ({
      status: selectContainerStatus(container.name as Container)(rootState),
      container,
    }))
    .filter(
      ({ container, status }) => container.eventsChannel && status.running,
    )
    .map(({ container }) => ({
      service: container.name,
      eventsChannel: container.eventsChannel,
    }))
