import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';

function MiningSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        Mining Settings
      </Typography>
      <SubHeading text="Mining" />
      <Typography sx={typography.defaultMedium}>
        Monero mining address
      </Typography>
      <TextField label="Mining Address" />
      <Typography variant="body1" style={typography.microMedium}>
        This is the address to which the Monero coins you earn will be sent. You
        need to provide a Monero address to be able to start Merged mining.{' '}
      </Typography>
    </>
  );
}

export default MiningSettings;
