import React, { useState, useEffect } from 'react';
import { Button, Box, Tabs, Dialog } from '@mui/material';
import useAppStateStore from '../../store/appStateStore';
import ThemeSwitch from '../../components/ThemeSwitch';
import MiningSettings from './MiningSettings/MiningSettings';
import BaseNodeSettings from './BaseNodeSettings/BaseNodeSettings';
import DockerSettings from './DockerSettings/DockerSettings';
import GeneralSettings from './GeneralSettings/GeneralSettings';
import WalletSettings from './WalletSettings/WalletSettings';
import ResetSettings from './ResetSettings/ResetSettings';
import { useTheme } from '@mui/material/styles';
import { HorisontalButtons } from '../../components/StyledComponents';
import {
  SettingsPanel,
  ScrollBarBox,
  SettingsTab,
  ThemeSwitchBox,
} from './styles';
import { emit } from '@tauri-apps/api/event';
import { TabPanelProps, FormDataType, FormDataSchema } from './types';

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

function SettingsDialog() {
  const [openSettings, setOpenSettings, appState, settingsTab] =
    useAppStateStore((state) => [
      state.openSettings,
      state.setOpenSettings,
      state.appState,
      state.settingsTab,
    ]);
  const [isValid, setIsValid] = useState(false);
  console.log('isValid', isValid);

  const settings = appState?.config?.settings?.saved_settings || {};

  const initialFormData = {
    miningSettings: {
      shaThreads: settings.sha3_miner?.num_mining_threads || 0,
      moneroAddress: settings.xmrig?.monero_mining_address || '',
      randomXThreads: settings.xmrig?.num_mining_threads || 0,
      moneroNodeUrl: settings.mm_proxy?.monerod_url || '',
      // walletPaymentAddress:
      //   settings.mm_proxy
      //     ?.wallet_payment_address ||
      //   settings.sha3_miner
      //     ?.wallet_payment_address ||
      //   '',
    },
    baseNodeSettings: {
      network: settings.tari_network || '',
      rootFolder: appState?.config?.settings?.data_directory || '',
    },
    walletSettings: {
      tariAddress:
        settings.mm_proxy?.wallet_payment_address ||
        settings.sha3_miner?.wallet_payment_address ||
        '',
    },
    dockerSettings: {
      dockerTag: appState?.config?.settings?.saved_settings?.tag || '',
      dockerRegistry:
        appState?.config?.settings?.saved_settings?.registry || '',
    },
    // generalSettings: {
    //   runOnStartup: false,
    //   mineOnStartup: false,
    // },
  };

  const [formData, setFormData] = useState<FormDataType>(initialFormData);
  const [isDirty, setIsDirty] = useState(false);
  const [value, setValue] = useState(settingsTab);
  const theme = useTheme();

  // console.log('formData', FormDataSchema.safeParse(formData));
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

  async function saveSettings(formData: FormDataType) {
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
    settings.xmrig.num_mining_threads = formData.miningSettings.randomXThreads;
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

    // DOCKER SETTINGS
    // dockerTag
    settings.tag = formData.dockerSettings.dockerTag;
    // dockerRegistry
    settings.registry = formData.dockerSettings.dockerRegistry;

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
          <ThemeSwitchBox>
            <ThemeSwitch />
          </ThemeSwitchBox>
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

export default SettingsDialog;
