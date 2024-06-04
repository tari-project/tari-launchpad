import { MessageBox } from '../../TBotContainer/styles';
import { useSnackbar } from 'notistack';
import { Button, Typography, List, ListItem } from '@mui/material';
import { StyledIconButton } from '../../../components/StyledComponents';
import SvgQuestion from '../../../styles/Icons/Question';
import typography from '../../../styles/styles/typography';
import t from '../../../locales';

export const BaseNodeHelpMessage = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const key = 'basenode-start-help';
  const handleClick = () => {
    enqueueSnackbar(
      <MessageBox>
        <Typography sx={typography.defaultMedium}>
          {t.baseNode.helpMessages.howItWorks.allowsYou}
        </Typography>
        <List>
          <ListItem>
            {t.baseNode.helpMessages.howItWorks.affordances[0]}
          </ListItem>
          <ListItem>
            {t.baseNode.helpMessages.howItWorks.affordances[1]}
          </ListItem>
          <ListItem>
            {t.baseNode.helpMessages.howItWorks.affordances[2]}
          </ListItem>
        </List>
        <Typography sx={typography.defaultMedium}>
          <span style={typography.defaultHeavy}>
            {t.baseNode.helpMessages.howItWorks.thankYou}
          </span>{' '}
          {t.baseNode.helpMessages.howItWorks.yourContribution}
        </Typography>
        <Button variant="contained" onClick={() => closeSnackbar(key)}>
          {t.common.phrases.gotIt}
        </Button>
      </MessageBox>,
      {
        key: key,
        persist: false,
      }
    );
  };

  return (
    <StyledIconButton onClick={handleClick}>
      <SvgQuestion />
    </StyledIconButton>
  );
};
