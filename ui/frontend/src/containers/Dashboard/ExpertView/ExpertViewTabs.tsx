import React, { useState } from 'react';
import { Typography, Tabs, Tab, Box } from '@mui/material';
import Containers from './Containers/Containers';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function CustomTabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`expert-tabpanel-${index}`}
      aria-labelledby={`expert-tab-${index}`}
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
    id: `expert-tab-${index}`,
    'aria-controls': `expert-tabpanel-${index}`,
  };
}

export default function ExpertViewTabs() {
  const [value, setValue] = useState(0);

  const handleChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  return (
    <Box sx={{ width: '100%', p: 5 }}>
      <Tabs
        TabIndicatorProps={{ style: { height: 4, borderRadius: 4 } }}
        value={value}
        onChange={handleChange}
        aria-label="Expert View Tabs"
        indicatorColor="secondary"
      >
        <Tab label="Containers" {...a11yProps(0)} />
      </Tabs>
      <CustomTabPanel value={value} index={0}>
        <Containers />
      </CustomTabPanel>
    </Box>
  );
}
