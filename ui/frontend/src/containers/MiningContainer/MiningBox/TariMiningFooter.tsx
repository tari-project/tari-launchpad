import SvgSetting2 from '../../../styles/Icons/Setting2';
import useAppStateStore from '../../../store/appStateStore';
import { FooterBox } from '../styles';
import {
  TextButton,
  LabelWithChip,
} from '../../../components/StyledComponents';
import { SettingsTabs } from '../../../store/types';

function TariMiningFooter() {
  const { openSettingsFunc } = useAppStateStore((state) => ({
    openSettingsFunc: state.openSettingsFunc,
  }));
  return (
    <FooterBox>
      <LabelWithChip>
        <TextButton
          variant="text"
          startIcon={<SvgSetting2 />}
          onClick={() => openSettingsFunc(SettingsTabs.SHA_MINING)}
          color="inherit"
        >
          Tari Mining Settings
        </TextButton>
      </LabelWithChip>
    </FooterBox>
  );
}

export default TariMiningFooter;
