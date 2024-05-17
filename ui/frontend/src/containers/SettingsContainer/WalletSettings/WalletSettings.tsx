import { Typography, TextField } from '@mui/material';
import typography from '../../../styles/styles/typography';
import { LabelBoxVertical } from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import t from '../../../locales';
import CopyToClipboard from '../../../components/CopyToClipboard';

function WalletSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: any;
}) {
  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {t.wallet.settings.title}
      </Typography>
      <SettingsBox>
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.wallet.wallet.walletId} ({t.wallet.wallet.address})
          </Typography>
          <TextField
            placeholder={t.wallet.wallet.walletId}
            name="walletSettings.tariAddress"
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
      </SettingsBox>
    </SettingsContainer>
  );
}

export default WalletSettings;
