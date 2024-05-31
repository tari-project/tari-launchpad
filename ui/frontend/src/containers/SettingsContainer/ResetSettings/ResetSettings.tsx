import { useState } from 'react';
import { Typography, Button, Box, Chip } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import {
  LabelWithChip,
  HorisontalButtons,
} from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import t from '../../../locales';

function ResetSettings() {
  const [confirmReset, setConfirmReset] = useState(false);
  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {t.reset.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text={t.reset.settings.subtitle} />
        <LabelWithChip>
          <Typography variant="body1" style={typography.defaultMedium}>
            {t.reset.settings.label}
          </Typography>
          <Chip label={t.reset.settings.warning} color="warning" />
        </LabelWithChip>

        <Typography variant="body2" style={typography.smallMedium}>
          {t.reset.settings.description}
        </Typography>
        <Box>
          {!confirmReset ? (
            <Button variant="contained" onClick={() => setConfirmReset(true)}>
              {t.reset.settings.resetButton}
            </Button>
          ) : (
            <HorisontalButtons>
              <Button variant="outlined" onClick={() => setConfirmReset(false)}>
                {t.reset.settings.keepEditing}
              </Button>
              <Button variant="contained" onClick={() => console.log('Reset')}>
                {t.reset.settings.resetAndExit}
              </Button>
            </HorisontalButtons>
          )}
        </Box>
      </SettingsBox>
    </SettingsContainer>
  );
}

export default ResetSettings;
