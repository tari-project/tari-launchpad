import { useState } from 'react'
import ResetSettings from './ResetSettings'
import { actions } from '../../../store/settings'
import { useAppDispatch } from '../../../store/hooks'

const ResetSettingsContainer = () => {
  const dispatch = useAppDispatch()
  const [confirmReset, setConfirmReset] = useState(false)

  const tryToReset = () => {
    setConfirmReset(true)
  }

  const resetSettings = () => {
    setConfirmReset(false)
    dispatch(actions.resetSettingsAndRelaunch())
  }

  return (
    <ResetSettings
      confirmReset={confirmReset}
      onReset={tryToReset}
      resetDiscard={() => setConfirmReset(false)}
      resetSettings={resetSettings}
    />
  )
}

export default ResetSettingsContainer
