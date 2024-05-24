import { Typography, TextField, Switch, Box } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import { LabelBoxVertical } from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import t from '../../../locales';
import CopyToClipboard from '../../../components/CopyToClipboard';
import { MergedMiningSettingsType } from '../types';
import { FormGroup, FormControlLabel } from '@mui/material';

function MergedMiningSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: MergedMiningSettingsType;
}) {
  const handleThreadChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value, 10);
    handleChange({
      target: {
        name: event.target.name,
        value: isNaN(value) ? '' : value,
      },
    });
  };

  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {/* {t.mining.settings.title} */}
        Merged Mining Settings
      </Typography>
      <SettingsBox>
        <SubHeading text={t.common.nouns.mergeMining} />
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.defaultMedium}>
            {t.mining.settings.moneroAddressLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.moneroAddressLabel}
            name="mergedMiningSettings.moneroAddress"
            value={formData.moneroAddress}
            onChange={handleChange}
            InputProps={{
              endAdornment: <CopyToClipboard copy={formData.moneroAddress} />,
            }}
          />
          <Typography variant="body2" style={typography.smallMedium}>
            {t.mining.settings.moneroAddressDesc1.regular}{' '}
            <span style={typography.smallHeavy}>
              {t.mining.settings.moneroAddressDesc1.bold}
            </span>
            <br />
            {t.mining.settings.moneroAddressDesc2.regular}{' '}
            <span style={typography.smallHeavy}>
              {t.mining.settings.moneroAddressDesc2.bold}
            </span>
          </Typography>
        </LabelBoxVertical>
        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                checked={formData.mergeMineOnStartup}
                onChange={() =>
                  handleChange({
                    target: {
                      name: 'mergedMiningSettings.mergeMineOnStartup',
                      value: !formData.mergeMineOnStartup,
                    },
                  })
                }
                inputProps={{ 'aria-label': 'start merged mining on startup' }}
                style={{
                  marginRight: '4px',
                }}
              />
            }
            label="Start merged mining on startup"
            labelPlacement="end"
          />
        </FormGroup>
        <SubHeading text={t.common.nouns.expert} />
        <Box
          style={{
            display: 'flex',
            flexDirection: 'row',
            justifyContent: 'space-between',
            width: '100%',
            gap: '16px',
          }}
        >
          <LabelBoxVertical>
            <Typography variant="body1" style={typography.defaultMedium}>
              {t.mining.settings.randomXThreadsLabel}
            </Typography>
            <TextField
              placeholder={t.mining.settings.randomXThreadsLabel}
              name="mergedMiningSettings.randomXThreads"
              value={formData.randomXThreads}
              onChange={handleThreadChange}
              style={{ maxWidth: 120 }}
            />
          </LabelBoxVertical>

          <LabelBoxVertical
            style={{
              flexGrow: 4,
            }}
          >
            <Typography variant="body1" style={typography.defaultMedium}>
              {t.mining.settings.moneroNodeUrlLabel}
            </Typography>
            <TextField
              placeholder={t.mining.settings.moneroNodeUrlLabel}
              name="mergedMiningSettings.moneroNodeUrl"
              value={formData.moneroNodeUrl}
              onChange={handleChange}
            />
          </LabelBoxVertical>
        </Box>
      </SettingsBox>
    </SettingsContainer>
  );
}

export default MergedMiningSettings;
