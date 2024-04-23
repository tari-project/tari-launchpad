import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import {
  SettingsBox,
  LabelBoxHorisontal,
  LabelBoxVertical,
} from '../UI/StyledComponents';
import t from '../../locales';

function MiningSettings() {
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
          <TextField placeholder={t.mining.settings.threadsLabel} />
        </LabelBoxHorisontal>
        <SubHeading text={t.common.nouns.mergeMining} />
        <LabelBoxVertical>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.mining.settings.moneroAddressLabel}
          </Typography>
          <TextField placeholder={t.mining.settings.moneroAddressLabel} />
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
        <TextField placeholder="RandomX Threads" />
        <TextField placeholder="Monero Node URL" />
        <TextField placeholder="Wallet Payment Address" />
      </SettingsBox>
    </>
  );
}

export default MiningSettings;
