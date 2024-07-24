import { useState } from 'react';
import { Typography, Button, Box, Chip, CircularProgress } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import {
  LabelWithChip,
  HorisontalButtons,
} from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import t from '../../../locales';

function AdvancedSettings({
  isZippingLogs,
  handleExportLogs,
}: {
  isZippingLogs: boolean,
  handleExportLogs: VoidFunction;
}) {
  const [confirmReset, setConfirmReset] = useState(false);
  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {t.advancedSettings.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text="Export logs" />

        <Typography variant="body2" style={typography.smallMedium}>
        {t.advancedSettings.settings.exportDescription}
        </Typography>

        <Box>
          {isZippingLogs ? <CircularProgress /> : <Button variant="contained" onClick={handleExportLogs}>
            {t.advancedSettings.settings.export}
          </Button>}
          
        </Box>
      </SettingsBox>
      <SettingsBox>
        <SubHeading text={t.advancedSettings.settings.subtitle} />
        <LabelWithChip>
          <Typography variant="body1" style={typography.defaultMedium}>
            {t.advancedSettings.settings.label}
          </Typography>
          <Chip label={t.advancedSettings.settings.warning} color="warning" />
        </LabelWithChip>

        <Typography variant="body2" style={typography.smallMedium}>
          {t.advancedSettings.settings.description}
        </Typography>
        <Box>
          {!confirmReset ? (
            <Button variant="contained" onClick={() => setConfirmReset(true)}>
              {t.advancedSettings.settings.resetButton}
            </Button>
          ) : (
            <HorisontalButtons>
              <Button variant="outlined" onClick={() => setConfirmReset(false)}>
                {t.advancedSettings.settings.keepEditing}
              </Button>
              <Button variant="contained" onClick={() => console.log('Reset')}>
                {t.advancedSettings.settings.resetAndExit}
              </Button>
            </HorisontalButtons>
          )}
        </Box>
      </SettingsBox>
    </SettingsContainer>
  );
}

export default AdvancedSettings;
