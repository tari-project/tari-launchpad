import { Typography, TextField, Switch } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import { LabelBoxHorisontal } from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import { BaseNodeSettingsType } from '../types';
import t from '../../../locales';
import { FormGroup, FormControlLabel } from '@mui/material';

function BaseNodeSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: BaseNodeSettingsType;
}) {
  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {t.baseNode.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text={t.baseNode.title} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.baseNode.tari_network_label}
          </Typography>
          <TextField
            placeholder={t.baseNode.tari_network_label}
            name="baseNodeSettings.network"
            value={formData.network}
            onChange={handleChange}
          />
        </LabelBoxHorisontal>
        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                checked={formData.runOnStartup}
                onChange={() =>
                  handleChange({
                    target: {
                      name: 'baseNodeSettings.runOnStartup',
                      value: !formData.runOnStartup,
                    },
                  })
                }
                inputProps={{ 'aria-label': 'run on startup' }}
                style={{
                  marginRight: '4px',
                }}
              />
            }
            label="Run on startup"
            labelPlacement="end"
          />
        </FormGroup>
        <SubHeading text={t.common.nouns.expert} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.baseNode.settings.rootFolder}
          </Typography>
          <TextField
            placeholder={t.baseNode.settings.rootFolder}
            name="baseNodeSettings.rootFolder"
            value={formData.rootFolder}
            onChange={handleChange}
            disabled
          />
        </LabelBoxHorisontal>
      </SettingsBox>
      <SettingsBox>
        <SubHeading text={t.baseNode.qrModal.title} />
      </SettingsBox>
    </SettingsContainer>
  );
}

export default BaseNodeSettings;
