import BaseNodeWidget from './BaseNodeWidget';
import BaseNodeFooter from './BaseNodeFooter';
import { TabContainer, ItemsContainer } from './styles';
import BaseNodeHelp from './BaseNodeHelp/BaseNodeHelp';

function BaseNodeTab() {
  return (
    <TabContainer>
      <BaseNodeHelp />
      <ItemsContainer>
        <BaseNodeWidget />
        <BaseNodeFooter />
      </ItemsContainer>
    </TabContainer>
  );
}

export default BaseNodeTab;
