import React, { useState, useEffect } from 'react';
import { Button, Box, Tab, Tabs, Dialog } from '@mui/material';
import useAppStateStore from '../../store/appStateStore';
import ThemeSwitch from '../../components/ThemeSwitch';
import MiningSettings from './MiningSettings/MiningSettings';
import BaseNodeSettings from './BaseNodeSettings/BaseNodeSettings';
import DockerSettings from './DockerSettings/DockerSettings';
import SecuritySettings from './SecuritySettings/SecuritySettings';
import GeneralSettings from './GeneralSettings/GeneralSettings';
import WalletSettings from './WalletSettings/WalletSettings';
import ResetSettings from './ResetSettings/ResetSettings';
import { styled, useTheme } from '@mui/material/styles';
import { HorisontalButtons } from '../../components/StyledComponents';
import { emit } from '@tauri-apps/api/event';
import { z } from 'zod';

const MiningSettingsSchema = z.object({
  shaThreads: z.number(),
  moneroAddress: z.string().min(8, 'Monero address is too short'),
  randomXThreads: z.number(),
  moneroNodeUrl: z.string(),
  // walletPaymentAddress: z.any(),
});

const BaseNodeSettingsSchema = z.object({
  network: z.string(),
  rootFolder: z.string(),
});

const WalletSettingsSchema = z.object({
  tariAddress: z.string(),
});

const DockerSettingsSchema = z.object({
  dockerTag: z.string(),
  dockerRegistry: z.string(),
});

// const GeneralSettingsSchema = z.object({
//   runOnStartup: z.boolean(),
//   mineOnStartup: z.boolean(),
// });

const FormDataSchema = z.object({
  miningSettings: MiningSettingsSchema,
  baseNodeSettings: BaseNodeSettingsSchema,
  walletSettings: WalletSettingsSchema,
  dockerSettings: DockerSettingsSchema,
  // generalSettings: GeneralSettingsSchema,
});

type MiningSettings = z.infer<typeof MiningSettingsSchema>;
type BaseNodeSettings = z.infer<typeof BaseNodeSettingsSchema>;
type WalletSettings = z.infer<typeof WalletSettingsSchema>;
type DockerSettings = z.infer<typeof DockerSettingsSchema>;
// type GeneralSettings = z.infer<typeof GeneralSettingsSchema>;
type FormData = z.infer<typeof FormDataSchema>;

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

function SettingsForm() {
  const [openSettings, setOpenSettings, appState] = useAppStateStore(
    (state) => [state.openSettings, state.setOpenSettings, state.appState]
  );
  const [isValid, setIsValid] = useState(false);

  const initialFormData = {
    miningSettings: {
      shaThreads:
        appState?.config?.settings?.saved_settings?.sha3_miner
          ?.num_mining_threads || 0,
      moneroAddress:
        appState?.config?.settings?.saved_settings?.xmrig
          ?.monero_mining_address || '',
      randomXThreads:
        appState?.config?.settings?.saved_settings?.sha3_miner
          ?.num_mining_threads || 0,
      moneroNodeUrl:
        appState?.config?.settings?.saved_settings?.mm_proxy?.monerod_url || '',
      // walletPaymentAddress:
      //   appState?.config?.settings?.saved_settings?.mm_proxy
      //     ?.wallet_payment_address ||
      //   appState?.config?.settings?.saved_settings?.sha3_miner
      //     ?.wallet_payment_address ||
      //   '',
    },
    baseNodeSettings: {
      network: appState?.config?.settings?.saved_settings?.tari_network || '',
      rootFolder: appState?.config?.settings?.data_directory || '',
    },
    walletSettings: {
      tariAddress:
        appState?.config?.settings?.saved_settings?.mm_proxy
          ?.wallet_payment_address ||
        appState?.config?.settings?.saved_settings?.sha3_miner
          ?.wallet_payment_address ||
        '',
    },
    dockerSettings: {
      dockerTag: 'dockerTag',
      dockerRegistry: 'dockerRegistry',
    },
    // generalSettings: {
    //   runOnStartup: false,
    //   mineOnStartup: false,
    // },
  };

  const [formData, setFormData] = useState<FormData>(initialFormData);
  const [isDirty, setIsDirty] = useState(false);
  const [value, setValue] = useState(0);
  const theme = useTheme();

  console.log('formData', FormDataSchema.safeParse(formData));
  const handleFormChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setIsDirty(true);

    const [section, field] = name.split('.');
    setFormData((prevData: any) => ({
      ...prevData,
      [section]: {
        ...prevData[section],
        [field]: value,
      },
    }));
  };

  const menuItems = [
    {
      label: 'Mining',
      component: (
        <MiningSettings
          handleChange={handleFormChange}
          formData={formData.miningSettings}
        />
      ),
    },
    {
      label: 'Base Node',
      component: (
        <BaseNodeSettings
          handleChange={handleFormChange}
          formData={formData.baseNodeSettings}
        />
      ),
    },
    {
      label: 'Wallet',
      component: (
        <WalletSettings
          handleChange={handleFormChange}
          formData={formData.walletSettings}
        />
      ),
    },
    {
      label: 'Docker',
      component: (
        <DockerSettings
          handleChange={handleFormChange}
          formData={formData.dockerSettings}
        />
      ),
    },
    {
      label: 'Security',
      component: <SecuritySettings />,
    },
    {
      label: 'General',
      component: <GeneralSettings />,
    },
    {
      label: 'Reset',
      component: <ResetSettings />,
    },
  ];

  const renderTab = menuItems.map((item, index) => {
    return <SettingsTab label={item.label} {...a11yProps(index)} key={index} />;
  });

  const renderTabPanel = menuItems.map((item, index) => {
    return (
      <TabPanel value={value} index={index} key={index}>
        {item.component}
      </TabPanel>
    );
  });

  const handleTabChange = (_event: React.SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const handleSave = () => {
    saveSettings(formData);
    setOpenSettings(false);
  };

  const handleCancel = () => {
    setFormData(initialFormData);
    setValue(0);
    setOpenSettings(false);
  };

  useEffect(() => {
    setFormData(initialFormData);
  }, [appState]);

  console.log('isValid', isValid);

  async function saveSettings(formData: FormData) {
    console.log('Save Form', FormDataSchema.safeParse(formData));
    const validatedData = FormDataSchema.safeParse(formData);
    if (!validatedData.success) {
      setIsValid(false);
      console.error('Validation Error', validatedData.error);
      return;
    } else {
      setIsValid(true);
    }

    const settings = { ...appState?.config?.settings?.saved_settings };
    // MINING SETTINGS
    // shaThreads
    settings.sha3_miner.num_mining_threads = formData.miningSettings.shaThreads;
    // moneroAddress
    settings.xmrig.monero_mining_address =
      formData.miningSettings.moneroAddress;
    // randomXThreads
    settings.sha3_miner.num_mining_threads =
      formData.miningSettings.randomXThreads;
    // moneroNodeUrl
    settings.mm_proxy.monerod_url = formData.miningSettings.moneroNodeUrl;
    // walletPaymentAddress
    // settings.sha3_miner.wallet_payment_address =
    //   formData.miningSettings.walletPaymentAddress;
    // settings.mm_proxy.wallet_payment_address =
    //   formData.miningSettings.walletPaymentAddress;

    // BASE NODE SETTINGS
    // network
    settings.tari_network = formData.baseNodeSettings.network;
    // //rootFolder
    // this breaks the interface
    // appState.config.settings = formData.baseNodeSettings.rootFolder;
    // tariAddress
    settings.sha3_miner.wallet_payment_address =
      formData.walletSettings.tariAddress;
    // tartAddress
    settings.mm_proxy.wallet_payment_address =
      formData.walletSettings.tariAddress;

    emit('tari://actions', {
      Action: { type: 'SaveSettings', payload: settings },
    });
  }

  return (
    <Dialog
      open={openSettings}
      onClose={handleCancel}
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
      fullWidth
      maxWidth="md"
    >
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
            onChange={handleTabChange}
            aria-label="Settings Tabs"
            TabIndicatorProps={{
              style: {
                display: 'none',
              },
            }}
            style={{
              width: 190,
              padding: `${theme.spacing(5)} 0 ${theme.spacing(
                2
              )} ${theme.spacing(3)}`,
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
      {/* Buttons */}
      <Box
        style={{
          display: 'flex',
          justifyContent: 'flex-end',
          padding: '16px',
          borderTop: `1px solid ${theme.palette.divider}`,
        }}
      >
        <HorisontalButtons>
          <Button variant="outlined" onClick={handleCancel}>
            Cancel
          </Button>
          <Button
            variant="contained"
            color="primary"
            onClick={handleSave}
            disabled={!isDirty}
          >
            Save
          </Button>
        </HorisontalButtons>
      </Box>
    </Dialog>
  );
}

export default SettingsForm;
