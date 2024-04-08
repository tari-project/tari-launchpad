import React, { useState } from 'react';
import { Typography, Tabs, Tab, Box, Chip } from '@mui/material';
import { useTheme } from '@mui/material/styles';
import MiningWidget from './MiningWidget';
import MergedMiningWidget from './MergedMiningWidget';
import useMiningStore from '../../store/miningStore';
import useMergedMiningStore from '../../store/mergedMiningStore';
import { StyledIconButton, TabInnerBox } from '../UI/StyledComponents';
import SvgQuestion from '../../styles/Icons/Question';
import typography from '../../styles/styles/typography';
import { useSnackbar } from 'notistack';

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
  const isMining = useMiningStore((state) => state.isMining);
  const isMergedMining = useMergedMiningStore((state) => state.isMergedMining);
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();

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
        TabIndicatorProps={{ style: { height: 3, borderRadius: 3 } }}
        value={value}
        onChange={handleChange}
        aria-label="Main page tabs"
      >
        <CustomTab
          label="Mining"
          isActive={isMining || isMergedMining}
          {...a11yProps(0)}
        />
        <Tab label="Base Node" {...a11yProps(1)} />
      </Tabs>
      <CustomTabPanel value={value} index={0}>
        <TabInnerBox>
          <Box
            style={{
              width: '100%',
              display: 'flex',
              flexDirection: 'column',
              gap: theme.spacing(3),
            }}
          >
            <Box
              style={{
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'center',
              }}
            >
              <Typography variant="h3" sx={typography.defaultHeavy}>
                You are one step away from starting mining. Want to know more
              </Typography>
              <StyledIconButton
                onClick={() =>
                  enqueueSnackbar(`Help message`, {
                    key: 1,
                    persist: true,
                  })
                }
              >
                <SvgQuestion />
              </StyledIconButton>
            </Box>
            <Box
              style={{
                display: 'grid',
                gap: theme.spacing(3),
                gridTemplateColumns: 'repeat(auto-fit, minmax(300px, 1fr))',
              }}
            >
              <MiningWidget />
              <MergedMiningWidget />
            </Box>
          </Box>
        </TabInnerBox>
      </CustomTabPanel>
      <CustomTabPanel value={value} index={1}>
        <Typography variant="h4">Base Node</Typography>
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
