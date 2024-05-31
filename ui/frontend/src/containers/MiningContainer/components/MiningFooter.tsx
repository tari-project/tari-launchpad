import SvgSetting2 from '../../../styles/Icons/Setting2';
import useAppStateStore from '../../../store/appStateStore';
import t from '../../../locales';
import { FooterBox } from '../styles';
// import SvgClock from '../../../styles/Icons/Clock';
// import SvgChart from '../../../styles/Icons/Chart';
import {
  TextButton,
  LabelWithChip,
} from '../../../components/StyledComponents';
import { Chip } from '@mui/material';
import { SettingsTabs } from '../../../store/types';

function MiningFooter() {
  const { openSettingsFunc } = useAppStateStore((state) => ({
    openSettingsFunc: state.openSettingsFunc,
  }));
  return (
    <FooterBox>
      {/* <TextButton
        variant="text"
        startIcon={<SvgClock />}
        onClick={() => setOpenSchedule(true)}
        color="inherit"
      >
        {t.mining.viewActions.setUpMiningHours}
      </TextButton> */}
      <LabelWithChip>
        <TextButton
          variant="text"
          startIcon={<SvgSetting2 />}
          onClick={() => openSettingsFunc(SettingsTabs.SHA_MINING)}
          color="inherit"
        >
          {t.mining.viewActions.miningSettings}
        </TextButton>
        <Chip label={t.common.nouns.expert} color="primary" />
      </LabelWithChip>
      {/* <TextButton
        variant="text"
        startIcon={<SvgChart />}
        onClick={() => console.log('statistics')}
        color="inherit"
      >
        {t.mining.viewActions.statistics}
      </TextButton> */}
    </FooterBox>
  );
}

export default MiningFooter;
