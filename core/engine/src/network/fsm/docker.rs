use crate::network::{DockerEvent, NetworkTaskFsm};
use bollard::system::EventsOptions;
use futures::StreamExt;
use std::collections::HashMap;
use tact::Receiver;

impl<'a> NetworkTaskFsm<'a> {
    pub fn subscribe_to_events(&mut self) {
        let mut type_filter = HashMap::new();
        type_filter.insert("type".to_string(), vec!["network".to_string()]);
        type_filter.insert("network".to_string(), vec![self.network().to_string()]);
        let opts = EventsOptions {
            since: None,
            until: None,
            filters: type_filter,
        };
        // TODO: Use the filter map and converter instead
        let stream = self.docker.events(Some(opts)).map(DockerEvent::from);
        let recipient = self.ctx.recipient();
        let receiver = Receiver::connect(stream, recipient);
        self.task.events = Some(receiver);
    }
}
