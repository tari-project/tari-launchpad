import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import { SettingsBox, LabelBoxHorisontal } from '../UI/StyledComponents';
import t from '../../locales';

function BaseNodeSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        {t.baseNode.settings.title}
      </Typography>
      <SettingsBox>
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.baseNode.tari_network_label}
          </Typography>
          <TextField placeholder={t.baseNode.tari_network_label} />
        </LabelBoxHorisontal>
        <SubHeading text={t.common.nouns.expert} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.baseNode.settings.rootFolder}
          </Typography>
          <TextField placeholder={t.baseNode.settings.rootFolder} />
        </LabelBoxHorisontal>
      </SettingsBox>
      <SettingsBox>
        <SubHeading text={t.baseNode.qrModal.title} />
      </SettingsBox>
    </>
  );
}

export default BaseNodeSettings;
