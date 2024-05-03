import React, { useState } from 'react';
import { Typography, Tabs, Tab, Box, Chip } from '@mui/material';
import { useTheme } from '@mui/material/styles';
import MiningTab from '../../MiningContainer/MiningTab';
import BaseNodeTab from '../../BaseNodeContainer/BaseNodeTab';
import useAppStateStore from '../../../store/appStateStore';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

interface CustomTabProps {
  isActive: boolean;
  label: string;
}

function CustomTabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`simple-tabpanel-${index}`}
      aria-labelledby={`simple-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ pt: 3, pb: 3 }}>
          <Typography>{children}</Typography>
        </Box>
      )}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `simple-tab-${index}`,
    'aria-controls': `simple-tabpanel-${index}`,
  };
}

export default function MainTabs() {
  const [value, setValue] = useState(0);
  const theme = useTheme();
  const { isMining, isMergeMining } = useAppStateStore();

  const handleChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const CustomTab = ({ isActive, label, ...props }: CustomTabProps) => (
    <Tab
      label={
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: theme.spacing(1),
          }}
        >
          <span>{label}</span>
          {isActive && (
            <Chip
              label={
                <span>
                  <strong>Running</strong> Mainnet
                </span>
              }
              color="success"
            />
          )}
        </div>
      }
      {...props}
    />
  );

  return (
    <Box sx={{ width: '100%' }}>
      <Tabs
        TabIndicatorProps={{ style: { height: 4, borderRadius: 4 } }}
        value={value}
        onChange={handleChange}
        aria-label="Main page tabs"
      >
        <CustomTab
          label="Mining"
          isActive={isMining || isMergeMining}
          {...a11yProps(0)}
        />
        <Tab label="Base Node" {...a11yProps(1)} />
      </Tabs>
      <CustomTabPanel value={value} index={0}>
        <MiningTab />
      </CustomTabPanel>
      <CustomTabPanel value={value} index={1}>
        <BaseNodeTab />
      </CustomTabPanel>
    </Box>
  );
}
