### The problem

We have a dispersed integration between the backend and frontend: a set of methods that called independently to control
images and containers. The state of running containers managed partially by both parts that makes debugging too
difficult.

Also we use multiple instances to interact with docker: `Workspaces`, `DockerWrapper`, an `AppState` with a `Docker`
instance inside the wrapper, and global standalone `Docker` instance.

There is no a single part responsible for managing containers. For example, we have to monitor docker's events to
subscribe for a GRPC stream from a wallet container that makes UI's code confusing and tricky for debugging.

### The solution

We need a single self-sufficient crate to manage a set of images and containers to maintain a node of the Tari Network.

That crate could provide a universal abstraction for controlling a deployment of Tari products (Base Node, Mining Node,
Wallet). The unified approach gives serious benefits:

- Easy unit tests coverage
- Compatibility with different systems (not only Docker, but also Kubernetes)
- Support connections to remote clusters
- CLI Launchpad (for testing purposes)
- Single synchronized state of containers
- One channel with events (not necessary to track connections for launched containers manually)
- Can be used for Web UI as well
- Can be a part of SaaS infrastructure (in case if we will decide to offer nodes as a service)

#### Implementation

The library can be implemented as a crate that has a method to get a stream of events and a method to send events.

The crate could work as a library and as a binary CLI tool.

#### Tauri Platform integration

For using the crate with the `tauri` platform we could add a method to subscribe to that stream of events that will be
forwarder to `tauri` events bus. Control events could be sent using a method call.
