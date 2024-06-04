import SvgSetting2 from '../../styles/Icons/Setting2';
import useAppStateStore from '../../store/appStateStore';
import t from '../../locales';
import { FooterBox } from './styles';
import { TextButton } from '../../components/StyledComponents';
import { SettingsTabs } from '../../store/types';

function BaseNodeFooter() {
  const { openSettingsFunc } = useAppStateStore((state) => ({
    openSettingsFunc: state.openSettingsFunc,
  }));
  return (
    <FooterBox>
      {/* <InfoBox>
        <Chip label={t.common.adjectives.recommended} color="info" />
        <Typography variant="body1" sx={typography.defaultMedium}>
          <span style={typography.defaultUnder}>
            {t.baseNode.aurora.connectYourAurora}
          </span>{' '}
          {t.baseNode.aurora.withBaseNode}
        </Typography>
        <Typography variant="body2" sx={typography.smallMedium}>
          {t.baseNode.aurora.description}
        </Typography>
      </InfoBox> */}
      <TextButton
        variant="text"
        startIcon={<SvgSetting2 />}
        onClick={() => openSettingsFunc(SettingsTabs.BASE_NODE)}
        color="inherit"
      >
        {t.baseNode.viewActions.baseNodeSettings}
      </TextButton>
    </FooterBox>
  );
}

export default BaseNodeFooter;
