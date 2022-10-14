import { useTheme } from 'styled-components'
import { RowFlex, DiscardWarning } from '../styles'
import Tag from '../../../components/Tag'
import Button from '../../../components/Button'

import SettingsSectionHeader from '../../../components/SettingsSectionHeader'
import Text from '../../../components/Text'
import t from '../../../locales'
import { SettingsHeader } from '../styles'

import { ResetSettingsInputs } from './../types'

const ResetSettings = ({
  confirmCancel,
  confirmReset,
  onReset,
  resetDiscard,
  resetSettings,
}: ResetSettingsInputs) => {
  const theme = useTheme()

  return (
    <>
      <SettingsHeader>
        <Text type='subheader' as='h2' color={theme.primary}>
          {t.reset.settings.title}
        </Text>
      </SettingsHeader>

      <SettingsSectionHeader noTopMargin noBottomMargin>
        {t.common.nouns.dangerZone}
      </SettingsSectionHeader>

      <RowFlex
        style={{
          marginTop: theme.spacingVertical(1),
          marginBottom: theme.spacingVertical(1),
        }}
      >
        <Text type='smallMedium' color={theme.primary}>
          {t.reset.settings.label}
        </Text>
        <Tag variant='small' type='warning'>
          {t.reset.settings.warning}
        </Tag>
      </RowFlex>
      <div
        style={{
          color: theme.secondary,
          marginBottom: theme.spacingVertical(1.5),
        }}
      >
        <Text type='smallMedium'>{t.reset.settings.description}</Text>
      </div>

      {confirmReset && (
        <div>
          <DiscardWarning>
            <Text type='smallHeavy'>{t.settings.resetChanges}</Text>
            <Text type='smallMedium'>{t.settings.resetChangesDesc}.</Text>
          </DiscardWarning>
          <Button variant='secondary' onClick={resetDiscard} size='small'>
            {t.common.phrases.keepEditing}
          </Button>
          <Button onClick={resetSettings} variant='warning' size='small'>
            {t.settings.resetAndExit}
          </Button>
        </div>
      )}
      {!confirmCancel && !confirmReset && (
        <div>
          <Button variant='secondary' onClick={onReset}>
            {t.common.verbs.reset}
          </Button>
        </div>
      )}
    </>
  )
}

export default ResetSettings
