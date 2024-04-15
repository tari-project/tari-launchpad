import { Typography, ListItem, Button, Box, List, Chip } from '@mui/material';
import typography from '../../styles/styles/typography';
import t from '../../locales';
import { SettingsBox } from '../UI/StyledComponents';
import { useTheme } from '@mui/material/styles';

function SecuritySettings() {
  const theme = useTheme();
  return (
    <SettingsBox>
      <Typography variant="h3" style={typography.subheader}>
        {t.settings.security.title}
      </Typography>
      <Box
        style={{
          display: 'flex',
          flexDirection: 'row',
          alignItems: 'center',
          gap: theme.spacing(1),
        }}
      >
        <Typography variant="body1" style={typography.defaultHeavy}>
          {t.settings.security.backupRecoveryPhrase}
        </Typography>
        <Chip label={t.common.adjectives.recommended} color="info" />
      </Box>
      <Typography variant="body1" style={typography.smallMedium}>
        {t.settings.security.tab.desc}
      </Typography>
      <List
        sx={{
          listStyleType: 'disc',
          pl: 2,
          '& .MuiListItem-root': {
            display: 'list-item',
          },
        }}
      >
        <ListItem>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.settings.security.tab.list1}
          </Typography>
        </ListItem>
        <ListItem>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.settings.security.tab.list2}
          </Typography>
        </ListItem>
        <ListItem>
          <Typography variant="body1" style={typography.smallMedium}>
            {t.settings.security.tab.list3}
          </Typography>
        </ListItem>
      </List>
      <Box>
        <Button variant="contained">
          {t.settings.security.createRecoveryPhrase}
        </Button>
      </Box>
    </SettingsBox>
  );
}

export default SecuritySettings;
