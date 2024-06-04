import MiningWidget from './MiningBox/MiningWidget';
import MergeMiningWidget from './MiningBoxMerged/MergeMiningWidget';
import TariMiningFooter from './MiningBox/TariMiningFooter';
import MergeMiningFooter from './MiningBoxMerged/MergeMiningFooter';
import { TabContainer, ItemsContainer } from './styles';
import MiningHelp from './MiningHelp/MiningHelp';

function MiningTab() {
  return (
    <TabContainer>
      <MiningHelp />
      <ItemsContainer>
        <MiningWidget />
        <MergeMiningWidget />
        <TariMiningFooter />
        <MergeMiningFooter />
      </ItemsContainer>
    </TabContainer>
  );
}

export default MiningTab;
