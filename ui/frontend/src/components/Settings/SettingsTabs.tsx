import * as React from 'react';
import { Tabs, Tab, Box } from '@mui/material';
import ThemeSwitcher from '../UI/ThemeSwitcher';
import MiningSettings from './MiningSettings';
import BaseNodeSettings from './BaseNodeSettings';
import DockerSettings from './DockerSettings';
import LogsSettings from './LogsSettings';
import SecuritySettings from './SecuritySettings';
import WalletSettings from './WalletSettings';
import { styled, useTheme } from '@mui/material/styles';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}
const SettingsTab = styled(Tab)(({ theme }) => ({
  borderRadius: '6px 0 0 6px',
  alignItems: 'flex-start',
  color: theme.palette.primary.dark,
  fontSize: '14px',
  '&.Mui-selected': {
    backgroundColor: theme.palette.divider,
    color: theme.palette.primary.main,
    fontFamily: '"AvenirHeavy", sans-serif',
  },
}));

const SettingsPanel = styled(Box)(({ theme }) => ({
  display: 'flex',
  height: 500,
  width: '100%',
  padding: `${theme.spacing(5)} ${theme.spacing(10)}`,
  flexDirection: 'column',
  gap: theme.spacing(3),
}));

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`vertical-tabpanel-${index}`}
      aria-labelledby={`vertical-tab-${index}`}
      {...other}
      style={{ width: '100%' }}
    >
      {value === index && <SettingsPanel>{children}</SettingsPanel>}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `vertical-tab-${index}`,
    'aria-controls': `vertical-tabpanel-${index}`,
  };
}

export default function SettingsTabs() {
  const [value, setValue] = React.useState(0);
  const theme = useTheme();

  const handleChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Box
      sx={{
        flexGrow: 1,
        display: 'flex',
        height: 500,
      }}
    >
      <Box
        sx={{
          borderRight: 1,
          borderColor: 'divider',
        }}
      >
        <Tabs
          orientation="vertical"
          variant="scrollable"
          value={value}
          onChange={handleChange}
          aria-label="Settings Tabs"
          sx={{
            width: 176,
            // paddingTop: theme.spacing(5),
            // paddingBottom: theme.spacing(5),
            padding: `${theme.spacing(5)} 0 ${theme.spacing(5)} ${theme.spacing(
              3
            )}`,
          }}
          TabIndicatorProps={{
            style: {
              display: 'none',
            },
          }}
        >
          <SettingsTab label="Mining" {...a11yProps(0)} />
          <SettingsTab label="Wallet" {...a11yProps(1)} />
          <SettingsTab label="Base Node" {...a11yProps(2)} />
          <SettingsTab label="Docker" {...a11yProps(3)} />
          <SettingsTab label="Logs" {...a11yProps(4)} />
          <SettingsTab label="Security" {...a11yProps(5)} />
        </Tabs>
        <ThemeSwitcher />
      </Box>
      <TabPanel value={value} index={0}>
        <MiningSettings />
      </TabPanel>
      <TabPanel value={value} index={1}>
        <WalletSettings />
      </TabPanel>
      <TabPanel value={value} index={2}>
        <BaseNodeSettings />
      </TabPanel>
      <TabPanel value={value} index={3}>
        <DockerSettings />
      </TabPanel>
      <TabPanel value={value} index={4}>
        <LogsSettings />
      </TabPanel>
      <TabPanel value={value} index={5}>
        <SecuritySettings />
      </TabPanel>
    </Box>
  );
}
