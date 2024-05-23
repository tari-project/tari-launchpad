import { Typography, TextField, Switch } from '@mui/material';
import typography from '../../../styles/styles/typography';
import SubHeading from '../../../components/SubHeading';
import {
  LabelBoxVertical,
  LabelBoxHorisontal,
} from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import t from '../../../locales';
import { ShaMiningSettingsType } from '../types';
import CopyToClipboard from '../../../components/CopyToClipboard';
import { FormGroup, FormControlLabel } from '@mui/material';

function ShaMiningSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: ShaMiningSettingsType;
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
        {/* {t.wallet.settings.title} */}
        Tari Mining Settings
      </Typography>
      <SettingsBox>
        <SubHeading text={t.common.nouns.shaMining} />
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.wallet.wallet.walletId} ({t.wallet.wallet.address})
          </Typography>
          <TextField
            placeholder={t.wallet.wallet.walletId}
            name="shaMiningSettings.tariAddress"
            value={formData.tariAddress}
            onChange={handleChange}
            InputProps={{
              endAdornment: <CopyToClipboard copy={formData.tariAddress} />,
            }}
          />
        </LabelBoxVertical>
        <Typography variant="body2" style={typography.smallMedium}>
          {t.wallet.settings.explanations.storage}{' '}
          {t.wallet.settings.explanations.send} (
          {t.wallet.settings.explanations.try}{' '}
          <span style={typography.smallUnder}>
            {t.wallet.settings.explanations.aurora}
          </span>{' '}
          {t.wallet.settings.explanations.itsGreat}){' '}
          {t.wallet.settings.explanations.extendedFunctionality}{' '}
          {t.wallet.settings.explanations.convert}{' '}
        </Typography>
        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                checked={formData.shaMineOnStartup}
                onChange={() =>
                  handleChange({
                    target: {
                      name: 'shaMiningSettings.shaMineOnStartup',
                      value: !formData.shaMineOnStartup,
                    },
                  })
                }
                inputProps={{ 'aria-label': 'start tari mining on startup' }}
                style={{
                  marginRight: '4px',
                }}
              />
            }
            label="Start Tari mining on startup"
            labelPlacement="end"
          />
        </FormGroup>
        <SubHeading text={t.common.nouns.expert} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.threadsLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.threadsLabel}
            name="shaMiningSettings.shaThreads"
            value={formData.shaThreads}
            onChange={handleThreadChange}
            style={{
              maxWidth: 120,
            }}
          />
        </LabelBoxHorisontal>
      </SettingsBox>
    </SettingsContainer>
  );
}

export default ShaMiningSettings;
