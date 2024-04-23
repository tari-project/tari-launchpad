import { Typography, ListItem, Button, Box, List, Chip } from '@mui/material';
import typography from '../../styles/styles/typography';
import { SettingsBox } from '../UI/StyledComponents';
import { useTheme } from '@mui/material/styles';
import t from '../../locales';

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
        <Typography variant="body1" style={typography.smallMedium}>
          {t.settings.security.backupRecoveryPhrase}
        </Typography>
        <Chip label={t.common.adjectives.recommended} color="info" />
      </Box>
      <Box>
        <Typography variant="body2" style={typography.smallMedium}>
          {t.settings.security.tab.desc}
        </Typography>
        <List
          sx={{
            listStyleType: 'disc',
            pl: 2,
            color: theme.palette.text.secondary,
            '& .MuiListItem-root': {
              display: 'list-item',
            },
          }}
          style={typography.smallMedium}
        >
          <ListItem style={{ padding: 0 }}>
            {t.settings.security.tab.list1}
          </ListItem>
          <ListItem style={{ padding: 0 }}>
            {t.settings.security.tab.list2}
          </ListItem>
          <ListItem style={{ padding: 0 }}>
            {t.settings.security.tab.list3}
          </ListItem>
        </List>
      </Box>
      <Box>
        <Button variant="contained">
          {t.settings.security.createRecoveryPhrase}
        </Button>
      </Box>
    </SettingsBox>
  );
}

export default SecuritySettings;
