import useThemeStore from '../../store/themeStore';
import { StyledIconButton } from './StyledComponents';
import SvgSun from '../../styles/Icons/Sun';
import SvgMoon from '../../styles/Icons/Moon';
import Switch from '@mui/material/Switch';
import { styled } from '@mui/material/styles';
import { Box, Typography } from '@mui/material';
import typography from '../../styles/styles/typography';

const SwitchContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  flexDirection: 'row',
  background: theme.palette.divider,
  width: 'fit-content',
  borderRadius: theme.shape.borderRadius,
}));

const IconStyle = {
  width: 20,
  height: 20,
};

const ThemeSwitch = () => {
  const { themeMode, setThemeMode } = useThemeStore();

  return (
    <Box>
      <Typography variant="body2" sx={typography.microMedium}>
        Select Theme
      </Typography>
      <SwitchContainer>
        <StyledIconButton onClick={() => setThemeMode('light')}>
          <SvgSun style={IconStyle} />
        </StyledIconButton>
        <Switch
          checked={themeMode === 'dark'}
          onChange={() =>
            setThemeMode(themeMode === 'light' ? 'dark' : 'light')
          }
          name="theme-switch"
          inputProps={{ 'aria-label': 'theme-switch' }}
        />
        <StyledIconButton onClick={() => setThemeMode('dark')}>
          <SvgMoon style={IconStyle} />
        </StyledIconButton>
      </SwitchContainer>
    </Box>
  );
};

export default ThemeSwitch;
