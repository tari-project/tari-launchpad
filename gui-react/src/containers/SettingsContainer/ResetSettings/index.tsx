import { useState } from 'react'
import { Control, FormState, UseFormSetValue } from 'react-hook-form'
import { SettingsInputs } from '../types'
import ResetSettings from './ResetSettings'
import { actions } from '../../../store/settings'
import { useAppDispatch } from '../../../store/hooks'

const ResetSettingsContainer = ({
  formState,
  control,
  values,
  setValue,
  setOpenMiningAuthForm,
}: {
  formState: FormState<SettingsInputs>
  control: Control<SettingsInputs>
  values: SettingsInputs
  setValue: UseFormSetValue<SettingsInputs>
  setOpenMiningAuthForm: (value: boolean) => void
}) => {
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
      confirmCancel={false}
      confirmReset={confirmReset}
      onReset={tryToReset}
      resetDiscard={() => setConfirmReset(false)}
      resetSettings={resetSettings}
    />
  )
}

export default ResetSettingsContainer
