import React, { useState } from 'react';
import { Button, Box, Tabs, Dialog } from '@mui/material';
import useAppStateStore from '../../store/appStateStore';
import ThemeSwitch from '../../components/ThemeSwitch';
import MergedMiningSettings from './MergedMiningSettings/MergedMiningSettings';
import BaseNodeSettings from './BaseNodeSettings/BaseNodeSettings';
import ShaMiningSettings from './ShaMiningSettings/ShaMiningSettings';
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
import useConfigStore from '../../store/configStore';
import { useShallow } from 'zustand/react/shallow';

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
  const { openSettings, setOpenSettings, appState, settingsTab } =
    useAppStateStore(
      useShallow((state) => ({
        openSettings: state.openSettings,
        setOpenSettings: state.setOpenSettings,
        appState: state.appState,
        settingsTab: state.settingsTab,
      }))
    );
  const { startupConfig, setStartupConfig } = useConfigStore();
  const [isValid, setIsValid] = useState(false);
  console.log('isValid', isValid);

  const settings = appState?.config?.settings?.saved_settings || {};

  const initialFormData = {
    mergedMiningSettings: {
      moneroAddress: settings.xmrig?.monero_mining_address || '',
      moneroNodeUrl: settings.mm_proxy?.monerod_url || '',
      mergeMineOnStartup: startupConfig.mergeMine || false,
    },
    baseNodeSettings: {
      network: settings.tari_network || '',
      rootFolder: appState?.config?.settings?.data_directory || '',
      runOnStartup: startupConfig.baseNode || false,
    },
    shaMiningSettings: {
      tariAddress:
        settings.mm_proxy?.wallet_payment_address ||
        settings.sha3_miner?.wallet_payment_address ||
        '',
      shaThreads: settings.sha3_miner?.num_mining_threads || 0,
      shaMineOnStartup: startupConfig.shaMine || false,
    },
    dockerSettings: {
      dockerTag: appState?.config?.settings?.saved_settings?.tag || '',
      dockerRegistry:
        appState?.config?.settings?.saved_settings?.registry || '',
    },
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

  const handleResetSettings = () => {
    emit('tari://actions', {
      Action: { type: 'ResetSettings' },
    });
  }

  const menuItems = [
    {
      label: 'Tari Mining',
      component: (
        <ShaMiningSettings
          handleChange={handleFormChange}
          formData={formData.shaMiningSettings}
        />
      ),
    },
    {
      label: 'Merged Mining',
      component: (
        <MergedMiningSettings
          handleChange={handleFormChange}
          formData={formData.mergedMiningSettings}
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
      label: 'Reset',
      component: <ResetSettings handleReset={handleResetSettings} />,
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
    settings.sha3_miner.num_mining_threads =
      formData.shaMiningSettings.shaThreads;
    // moneroAddress
    settings.xmrig.monero_mining_address =
      formData.mergedMiningSettings.moneroAddress;
    // moneroNodeUrl
    settings.mm_proxy.monerod_url = formData.mergedMiningSettings.moneroNodeUrl;
    // sha mine on startup
    setStartupConfig('shaMine', formData.shaMiningSettings.shaMineOnStartup);

    // MERGE MINING SETTINGS
    // merge mine on startup
    setStartupConfig(
      'mergeMine',
      formData.mergedMiningSettings.mergeMineOnStartup
    );

    // walletPaymentAddress
    // settings.sha3_miner.wallet_payment_address =
    //   formData.miningSettings.walletPaymentAddress;
    // settings.mm_proxy.wallet_payment_address =
    //   formData.miningSettings.walletPaymentAddress;

    // BASE NODE SETTINGS
    // network
    settings.tari_network = formData.baseNodeSettings.network;
    // runOnStartup
    setStartupConfig('baseNode', formData.baseNodeSettings.runOnStartup);
    // //rootFolder
    // this breaks the interface
    // appState.config.settings = formData.baseNodeSettings.rootFolder;
    // tariAddress
    settings.sha3_miner.wallet_payment_address =
      formData.shaMiningSettings.tariAddress;
    // tartAddress
    settings.mm_proxy.wallet_payment_address =
      formData.shaMiningSettings.tariAddress;

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
