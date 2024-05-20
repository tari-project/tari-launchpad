import { StyledIconButton } from '../../components/StyledComponents';
import { RiRobot2Line } from 'react-icons/ri';
import { IoCloseOutline } from 'react-icons/io5';
import { useSnackbar } from 'notistack';
import { useTheme } from '@mui/material/styles';
import { useState } from 'react';

export function SnackbarCloseButton({ snackbarKey }: any) {
  const { closeSnackbar } = useSnackbar();
  const theme = useTheme();

  return (
    <StyledIconButton onClick={() => closeSnackbar(snackbarKey)}>
      <IoCloseOutline
        style={{
          color: theme.palette.text.primary,
        }}
      />
    </StyledIconButton>
  );
}

const TBot = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const [count, setCount] = useState(1);

  const handleClick = () => {
    enqueueSnackbar(`🐸 Ribbit ribbit ${count}`, {
      key: count,
      persist: true,
    });
    setCount(count + 1);
  };

  return (
    <>
      <StyledIconButton onClick={handleClick}>
        <RiRobot2Line />
      </StyledIconButton>

      <StyledIconButton onClick={() => closeSnackbar()}>
        <IoCloseOutline />
      </StyledIconButton>
    </>
  );
};

export default TBot;
