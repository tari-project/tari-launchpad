import useThemeStore from '../store/themeStore';
import { StyledIconButton } from './StyledComponents';
import SvgSun from '../styles/Icons/Sun';
import SvgMoon from '../styles/Icons/Moon';

const ThemeSwitcher = () => {
  const { themeMode, setThemeMode } = useThemeStore();

  return (
    <StyledIconButton
      onClick={() => setThemeMode(themeMode === 'light' ? 'dark' : 'light')}
    >
      {themeMode === 'light' ? <SvgMoon /> : <SvgSun />}
    </StyledIconButton>
  );
};

export default ThemeSwitcher;
