import { Typography, TextField } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import { LabelBoxHorisontal } from '../../../components/StyledComponents';
import { SettingsBox, SettingsContainer } from '../styles';
import typography from '../../../styles/styles/typography';
import { DockerSettingsType } from '../types';
import t from '../../../locales';

function DockerSettings({
  handleChange,
  formData,
}: {
  handleChange: any;
  formData: DockerSettingsType;
}) {
  return (
    <SettingsContainer>
      <Typography variant="h3" style={typography.subheader}>
        {t.docker.settings.title}
      </Typography>
      <SettingsBox>
        <SubHeading text={t.common.nouns.expert} />
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.defaultMedium}>
            {t.docker.settings.tagLabel}
          </Typography>
          <TextField
            placeholder={t.docker.settings.tagLabel}
            name="dockerSettings.dockerTag"
            value={formData.dockerTag}
            onChange={handleChange}
          />
        </LabelBoxHorisontal>
        <LabelBoxHorisontal>
          <Typography variant="body1" style={typography.defaultMedium}>
            {t.docker.settings.registryLabel}
          </Typography>
          <TextField
            placeholder={t.docker.settings.registryLabel}
            name="dockerSettings.dockerRegistry"
            value={formData.dockerRegistry}
            onChange={handleChange}
          />
        </LabelBoxHorisontal>
        <SubHeading text={t.docker.settings.imageStatuses} />
      </SettingsBox>
    </SettingsContainer>
  );
}

export default DockerSettings;
