import React, { useState } from 'react';
import { Typography, Tabs, Tab, Box, Chip } from '@mui/material';
import { useTheme } from '@mui/material/styles';
import MiningTab from '../../MiningContainer/MiningTab';
import BaseNodeTab from '../../BaseNodeContainer/BaseNodeTab';
import useAppStateStore from '../../../store/appStateStore';
import {
  BaseNodeStatus,
  ShaMiningStatus,
  MergeMiningStatus,
} from '../../../store/types';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

interface CustomTabProps {
  chip: React.ReactNode;
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
  const { containers, appState } = useAppStateStore();

  const handleChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const BaseNodeChip = () => {
    if (containers?.baseNode?.status === BaseNodeStatus.ACTIVE) {
      return (
        <Chip
          label={
            <span>
              <strong>Running</strong>{' '}
              {appState?.config?.settings?.saved_settings?.tari_network || ''}
            </span>
          }
          color="success"
        />
      );
    } else {
      return null;
    }
  };

  const MiningChip = () => {
    if (
      containers?.sha3Miner?.status === ShaMiningStatus.ACTIVE ||
      containers?.mmProxy?.status === MergeMiningStatus.ACTIVE
    ) {
      return (
        <Chip
          label={
            <span>
              <strong>Running</strong>
            </span>
          }
          color="success"
        />
      );
    } else {
      return null;
    }
  };

  const CustomTab = ({ chip, label, ...props }: CustomTabProps) => (
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
          {chip}
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
        <CustomTab label="Mining" chip={<MiningChip />} {...a11yProps(0)} />
        <CustomTab
          label="Base Node"
          chip={<BaseNodeChip />}
          {...a11yProps(1)}
        />
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
