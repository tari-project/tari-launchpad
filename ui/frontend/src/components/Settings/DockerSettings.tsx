import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import { SettingsBox, LabelBoxHorisontal } from '../UI/StyledComponents';
import typography from '../../styles/styles/typography';
import t from '../../locales';

function DockerSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        {t.docker.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text={t.common.nouns.expert} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.docker.settings.tagLabel}
          </Typography>
          <TextField placeholder={t.docker.settings.tagLabel} />
        </LabelBoxHorisontal>
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.docker.settings.registryLabel}
          </Typography>
          <TextField placeholder={t.docker.settings.registryLabel} />
        </LabelBoxHorisontal>
        <SubHeading text={t.docker.settings.imageStatuses} />
      </SettingsBox>
    </>
  );
}

export default DockerSettings;
