import * as React from 'react';
import { Tabs, Tab, Box } from '@mui/material';
import ThemeSwitch from '../UI/ThemeSwitch';
import MiningSettings from './MiningSettings';
import BaseNodeSettings from './BaseNodeSettings';
import DockerSettings from './DockerSettings';
import SecuritySettings from './SecuritySettings';
import GeneralSettings from './GeneralSettings';
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
  gap: theme.spacing(5),
}));

const ScrollBarBox = styled(Box)(({ theme }) => ({
  overflowY: 'scroll',
  scrollbarWidth: 'thin',
  scrollbarColor:
    theme.palette.mode === 'light'
      ? `${theme.palette.grey[300]} transparent`
      : `${theme.palette.grey[800]} transparent`,
  '&::-webkit-scrollbar': {
    width: '8px',
  },
  '&::-webkit-scrollbar-track': {
    backgroundColor: 'transparent',
  },
  '&::-webkit-scrollbar-thumb': {
    backgroundColor:
      theme.palette.mode === 'light'
        ? theme.palette.grey[300]
        : theme.palette.grey[800],
    borderRadius: '4px',
  },
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

  const menuItems = [
    {
      label: 'Mining',
      component: MiningSettings,
    },
    {
      label: 'Base Node',
      component: BaseNodeSettings,
    },
    {
      label: 'Wallet',
      component: WalletSettings,
    },
    {
      label: 'Docker',
      component: DockerSettings,
    },
    {
      label: 'Security',
      component: SecuritySettings,
    },
    {
      label: 'General',
      component: GeneralSettings,
    },
  ];

  const handleChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const renderTab = menuItems.map((item, index) => {
    return <SettingsTab label={item.label} {...a11yProps(index)} />;
  });

  const renderTabPanel = menuItems.map((item, index) => {
    return (
      <TabPanel value={value} index={index} key={index}>
        <item.component />
      </TabPanel>
    );
  });

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
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          justifyContent: 'space-between',
        }}
      >
        <Tabs
          orientation="vertical"
          variant="scrollable"
          value={value}
          onChange={handleChange}
          aria-label="Settings Tabs"
          TabIndicatorProps={{
            style: {
              display: 'none',
            },
          }}
          style={{
            width: 190,
            padding: `${theme.spacing(5)} 0 ${theme.spacing(2)} ${theme.spacing(
              3
            )}`,
          }}
        >
          {renderTab}
        </Tabs>
        <Box
          style={{
            padding: `${theme.spacing(2)} ${theme.spacing(3)}`,
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
          }}
        >
          <ThemeSwitch />
        </Box>
      </Box>
      <Box
        style={{
          width: '100%',
          overflow: 'hidden',
        }}
      >
        <ScrollBarBox>{renderTabPanel}</ScrollBarBox>
      </Box>
    </Box>
  );
}
