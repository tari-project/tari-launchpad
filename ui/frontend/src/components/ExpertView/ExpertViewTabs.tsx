import React, { useState } from 'react';
import { Typography, Tabs, Tab, Box } from '@mui/material';
import Containers from './Containers';

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
        TabIndicatorProps={{ style: { height: 3, borderRadius: 3 } }}
        value={value}
        onChange={handleChange}
        aria-label="Expert View Tabs"
        indicatorColor="secondary"
        // variant="fullWidth"
      >
        <Tab label="Performance" {...a11yProps(0)} />
        <Tab label="Containers" {...a11yProps(1)} />
        <Tab label="Logs" {...a11yProps(2)} />
      </Tabs>
      <CustomTabPanel value={value} index={0}>
        <Typography variant="h4">Performance</Typography>
        <Typography>
          Lorem ipsum dolor, sit amet consectetur adipisicing elit. Repudiandae
          aliquam recusandae dolorem, praesentium facilis consectetur animi
          autem assumenda laboriosam ratione dolore corrupti doloribus hic
          necessitatibus non in possimus, placeat totam? Lorem ipsum dolor, sit
          amet consectetur adipisicing elit. Repudiandae aliquam recusandae
          dolorem, praesentium facilis consectetur animi autem assumenda
          laboriosam ratione dolore corrupti doloribus hic necessitatibus non in
          possimus, placeat totam? Lorem ipsum dolor, sit amet consectetur
          adipisicing elit. Repudiandae aliquam recusandae dolorem, praesentium
          facilis consectetur animi autem assumenda laboriosam ratione dolore
          corrupti doloribus hic necessitatibus non in possimus, placeat totam?
          Lorem ipsum dolor, sit amet consectetur adipisicing elit. Repudiandae
          aliquam recusandae dolorem, praesentium facilis consectetur animi
          autem assumenda laboriosam ratione dolore corrupti doloribus hic
          necessitatibus non in possimus, placeat totam?
        </Typography>
      </CustomTabPanel>
      <CustomTabPanel value={value} index={1}>
        <Containers />
      </CustomTabPanel>
      <CustomTabPanel value={value} index={2}>
        <Typography variant="h4">Logs</Typography>
        <Typography>
          Lorem ipsum dolor, sit amet consectetur adipisicing elit. Repudiandae
          aliquam recusandae dolorem, praesentium facilis consectetur animi
          autem assumenda laboriosam ratione dolore corrupti doloribus hic
          necessitatibus non in possimus, placeat totam? Lorem ipsum dolor, sit
          amet consectetur adipisicing elit. Repudiandae aliquam recusandae
          dolorem, praesentium facilis consectetur animi autem assumenda
          laboriosam ratione dolore corrupti doloribus hic necessitatibus non in
          possimus, placeat totam? Lorem ipsum dolor, sit amet consectetur
          adipisicing elit. Repudiandae aliquam recusandae dolorem, praesentium
          facilis consectetur animi autem assumenda laboriosam ratione dolore
          corrupti doloribus hic necessitatibus non in possimus, placeat totam?
          Lorem ipsum dolor, sit amet consectetur adipisicing elit. Repudiandae
          aliquam recusandae dolorem, praesentium facilis consectetur animi
          autem assumenda laboriosam ratione dolore corrupti doloribus hic
          necessitatibus non in possimus, placeat totam?
        </Typography>
      </CustomTabPanel>
    </Box>
  );
}
