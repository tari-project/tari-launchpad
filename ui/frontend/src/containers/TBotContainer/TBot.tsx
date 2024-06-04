import { StyledIconButton } from '../../components/StyledComponents';
import { RiRobot2Line } from 'react-icons/ri';
import { IoCloseOutline } from 'react-icons/io5';
import { useSnackbar } from 'notistack';
import { useTheme } from '@mui/material/styles';
import { useState } from 'react';
import { Box, Typography, Button } from '@mui/material';
import typography from '../../styles/styles/typography';
import { MessageBox } from './styles';

export function SnackbarCloseButton({ snackbarKey }: any) {
  const { closeSnackbar } = useSnackbar();
  const theme = useTheme();

  return (
    <Box style={{ position: 'absolute', top: '10px', right: '10px' }}>
      <StyledIconButton onClick={() => closeSnackbar(snackbarKey)}>
        <IoCloseOutline
          style={{
            color: theme.palette.text.primary,
          }}
        />
      </StyledIconButton>
    </Box>
  );
}

const TBot = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const [count, setCount] = useState(1);

  const handleClick = () => {
    enqueueSnackbar(
      <MessageBox>
        <Typography sx={typography.subheader}>
          Ribbit ribbit {count} 🐸
        </Typography>
        <Typography sx={typography.defaultMedium}>
          This is a custom message with a close button.
        </Typography>
        <Button variant="contained" onClick={() => closeSnackbar(count)}>
          Got it
        </Button>
      </MessageBox>,
      {
        key: count,
        persist: true,
      }
    );
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
