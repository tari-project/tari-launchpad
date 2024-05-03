import { Typography, TextField } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import {
  SettingsBox,
  LabelBoxHorisontal,
  LabelBoxVertical,
} from '../../../components/StyledComponents';
import t from '../../../locales';
import CopyToClipboard from '../../../components/CopyToClipboard';

function MiningSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: any;
}) {
  console.log(typeof formData.shaThreads);
  const handleThreadChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value, 10); // Parse the input value to an integer
    handleChange({
      target: {
        name: event.target.name,
        value: isNaN(value) ? '' : value, // If NaN (not a number), set it as an empty string, otherwise set the parsed integer
      },
    });
  };
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        {t.mining.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text={t.common.nouns.shaMining} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.threadsLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.threadsLabel}
            name="miningSettings.shaThreads"
            value={formData.shaThreads}
            onChange={handleThreadChange}
          />
        </LabelBoxHorisontal>
        <SubHeading text={t.common.nouns.mergeMining} />
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.moneroAddressLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.moneroAddressLabel}
            name="miningSettings.moneroAddress"
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
        <SubHeading text={t.common.nouns.expert} />

        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.randomXThreadsLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.randomXThreadsLabel}
            name="miningSettings.randomXThreads"
            value={formData.randomXThreads}
            onChange={handleThreadChange}
          />
        </LabelBoxVertical>

        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.moneroNodeUrlLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.moneroNodeUrlLabel}
            name="miningSettings.moneroNodeUrl"
            value={formData.moneroNodeUrl}
            onChange={handleChange}
          />
        </LabelBoxVertical>

        {/* <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.walletPaymentAddressLabel}
          </Typography>
          <TextField
            placeholder={t.mining.settings.walletPaymentAddressLabel}
            name="miningSettings.walletPaymentAddress"
            value={formData.walletPaymentAddress}
            onChange={handleChange}
          />
        </LabelBoxVertical> */}
      </SettingsBox>
    </>
  );
}

export default MiningSettings;
