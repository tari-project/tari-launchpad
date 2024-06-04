import { AmountBox } from '../styles';
import { Typography } from '@mui/material';
import colors from '../../../styles/styles/colors';
import typography from '../../../styles/styles/typography';
import { CircularProgressLight } from '../../../components/StyledComponents';

function Amount({ amount }: { amount: number }) {
  return (
    <AmountBox>
      <CircularProgressLight size={20} thickness={5} />
      <Typography
        sx={typography.subheader}
        style={{
          color: colors.light.textSecondary,
        }}
      >
        {amount}
      </Typography>
      <Typography
        sx={typography.smallMedium}
        style={{
          color: colors.light.textSecondary,
        }}
      >
        XTR
      </Typography>
    </AmountBox>
  );
}

export default Amount;
