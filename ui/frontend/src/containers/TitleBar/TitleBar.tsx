import { useState } from 'react';
import { appWindow } from '@tauri-apps/api/window';
import { Stack } from '@mui/material';
import { IoClose, IoRemove } from 'react-icons/io5';
import { RiExpandUpDownFill, RiContractUpDownFill } from 'react-icons/ri';
import {
  CloseButton,
  MinimizeButton,
  ToggleButton,
  MinMaxStyle,
  TitleBarContainer,
} from './styles';
import TariLogo from '../../assets/tari-logo';
import HeaderButtons from './HeaderButtons';
import { useTheme } from '@mui/material/styles';

const TitleBar = ({
  open,
  handleDrawerClose,
  handleDrawerOpen,
  fullScreen,
}: {
  open: boolean;
  handleDrawerClose: () => void;
  handleDrawerOpen: () => void;
  fullScreen: boolean;
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const theme = useTheme();
  const minimize = () => appWindow.minimize();
  const close = () => appWindow.close();
  const toggleMaximize = () => {
    setIsExpanded(!isExpanded);
    appWindow.toggleMaximize();
  };

  return (
    <TitleBarContainer data-tauri-drag-region>
      <Stack direction="row" spacing={1} padding={1} alignItems={'center'}>
        {fullScreen ? null : (
          <>
            <Stack direction="row" spacing={1} padding={1}>
              <CloseButton onClick={close}>
                <IoClose />
              </CloseButton>
              <MinimizeButton onClick={minimize}>
                <IoRemove />
              </MinimizeButton>
              <ToggleButton onClick={toggleMaximize}>
                {isExpanded ? (
                  <RiContractUpDownFill style={MinMaxStyle} />
                ) : (
                  <RiExpandUpDownFill style={MinMaxStyle} />
                )}
              </ToggleButton>
            </Stack>
            <Stack direction="row" spacing={1} padding={1}>
              <TariLogo fill={theme.palette.text.primary} />
            </Stack>
          </>
        )}
      </Stack>
      <HeaderButtons
        open={open}
        handleDrawerClose={handleDrawerClose}
        handleDrawerOpen={handleDrawerOpen}
      />
    </TitleBarContainer>
  );
};

export default TitleBar;
