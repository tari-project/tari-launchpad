# The Launchpad

Tari Launchpad is a tool to run a stack of Tari products simply:
ideally, the application should work automatically with minimal
user intervention.

To achieve high reliability, it is necessary to reduce the number
of exceptional cases, for example, to open a GRPC connection precisely
at the moment when the GRPC server is available or, for example,
to notify about all events occurring in containers with products
so that there are no silent errors that will be very difficult
for unsophisticated users to solve.

Today we already have an implementation that works, although it
has many bugs that are not easy to solve.

It is crucial to provide users the reliable product because:

1. The app will be the first experience for many new users,
and any bugs in that will give the impression of low reliability
of the network;
2. Competitors offer users similar apps, and the quality of our
product can't be less than the quality of their products.

Please note that I omitted to compare the app's functionality with
competitors since that parameter is not critical for the initial release.
Still, the app's stability directly determines the client's life cycle:
how long someone will want to use the product.

## The original architecture

The current implementation is designed for the [Tauri](https://tauri.app/)
platform and actively using API features of that.

The Launchpad app (commit [9859237][9859237]) consists of two parts:
`backend` and `gui-react`.

[9859237]: https://github.com/tari-project/tari-launchpad/commit/9859237619ae670bf2e15c10d336179651a12265

### The backend

The `backend` implemented with Rust and exports API methods using Tauri
platform's capabilities. The following list of methods is available for
the `gui-react` part:

```rust
// api/base_node_api.rs
async fn base_node_sync_progress(app: AppHandle<Wry>) -> Result<(), String>
async fn node_identity() -> Result<BaseNodeIdentity, String>

// api/mod.rs
fn network_list() -> Vec<String>
async fn image_info(settings: ServiceSettings) -> ImageListDto
async fn health_check(image: String) -> String

// api/wallet_api.rs
async fn wallet_identity() -> Result<WalletIdentity, String>
async fn wallet_balance() -> Result<WalletBalance, String>
async fn wallet_events(app: AppHandle<Wry>) -> Result<(), String>
async fn transfer(app: AppHandle<Wry>, funds: TransferFunds) -> Result<TransferFundsResult, String>
async fn get_seed_words(app: AppHandle<Wry>) -> Result<Vec<String>, String>
async fn delete_seed_words(app: AppHandle<Wry>) -> Result<(), String>
async fn transaction_fee() -> Result<u32, String>

// api/cleanup.rs
async fn clean_docker(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<(), String>

// api/create_workspace.rs
fn create_new_workspace(app: AppHandle<Wry>, root_path: Option<String>) -> Result<(), String>

// api/events.rs
async fn events(app: AppHandle<Wry>) -> Result<(), ()>

// api/host.rs
async fn check_docker() -> Result<Version, String>
async fn open_terminal(_app: AppHandle<Wry>, platform: String) -> Result<(), String>
async fn check_internet_connection() -> Result<bool, String>

// api/launch_docker.rs
async fn launch_docker(app: AppHandle<Wry>, name: String, config: WorkspaceLaunchOptions) -> Result<(), String>

// api/pull_images.rs
async fn pull_images(app: AppHandle<Wry>) -> Result<(), String>
async fn pull_image(app: AppHandle<Wry>, image_name: &str) -> Result<(), String>

// api/service.rs
async fn start_service(
    app: AppHandle<Wry>,
    service_name: String,
    settings: ServiceSettings,
) -> Result<StartServiceResult, String>
async fn stop_service(state: State<'_, AppState>, service_name: String) -> Result<(), String>
async fn create_default_workspace(app: AppHandle<Wry>, settings: ServiceSettings) -> Result<bool, String>

// api/shutdown.rs
async fn shutdown(state: State<'_, AppState>) -> Result<String, ()>
```

All the methods above, in most cases, are adapted methods of Docker.
For example, the `pull_image` method pulls an image with a specific name,
and `start_service` starts a particular container. To get events from containers,
we use the `events` method.

A vast number of errors appear because the frontend part has not considered
all the exceptional cases: for example, when grpc is not started yet, but
the frontend tried to connect to it. Or if the **Tor** container is not started,
but the frontend tried to launch the **Base Node**, that failed with an error
because it couldn't connect to the **Tor** server.

For the last problem, we have a naive solution that is not solved
the problem entirely: we added a `WAIT_FOR_TOR` environment variable
in the **Base Node**, but I often see cases when the tor runs longer
than that interval expected.

The obvious solution to this problem is: let's just take into account
all these cases in the frontend part. But how to test all that? To cover
that cases with tests, we have to create a lot of fragile tests that will
break simply because we move the buttons to other places.

Another potential problem is the destruction of the main window since
the frontend part controls the backend. If the main window is destroyed
accidentally by the system, we lose the state, and the containers
become unmanaged.

It is also impossible to create a multi-window interface, or
a container that manages others (a kind of launchpad for servers).

Another class of potential problems is related to the fact that we,
for example, send SQL queries directly from the frontend to the backend.
And, in theory, we have to cover that requests with tests as well.

### The frontend

The frontend uses `React` framework and `Redux` library.

The `Redux` maintains the main state of the app and calls API methods
that we exported from the backend.

## The improved architecture

The approach discussed below requires a lot of effort, but it can
significantly increase the reliability of the launchpad application.

The idea is to move all control functions to the backend and transfer
the application state and changes to this state to the frontend using
Tauri events.

In this case, in general, you will not need to export your API methods
since the application will also be controlled by sending messages from
frontend to backend, which will launch processes that change that
mentioned above - shared state.

### Events channels

To connect UI to the new engine we should use two calls on the frontend that is
the part of the Tauri API.

To subscribe to the state and changes of that we should subscribe to the
`tari://reactions` stream of events:

```js
const unlisten = await listen('tari://reactions', (event) => { })
```

And to send a control action we should send an event to the `tari://actions`
stream:

```js
function sendEvent(action) {
    emit('tari://actions', action)
}
```

The reasonable questions may occur: if we send actions and have to wait
for the state to be updated by a backend and a change event will be sent
and applied to the state in the UI, will we have a significant delay
noticeable to the user?

The answer is no; we don't have too long delays because it's
a desktop app, and it takes up to 2ms to send an event from
the backend to a server.

### The state

The state is the main object exists in the backend and has to be
copied to the frontend.

The state is a struct with:

- a configuration of the app
- a map of the states of the containers
- a state of the wallet

```rust
pub struct LaunchpadState {
    pub config: Option<LaunchpadConfig>,
    pub containers: HashMap<TaskId, TaskState>,
    pub wallet: WalletState,
}
```

The state above should be declared in the frontend part and received from the
`tari://reactions` stream that send events of the following `Reaction` type:

```rust
pub enum Reaction {
    State(LaunchpadState),
    Delta(LaunchpadDelta),
}
```

As you can see the event could be the whole state of a delta event that has
to be applied to the state received before.

The delta is represented be the following struct:

```rust
pub enum LaunchpadDelta {
    /// A new configuration is loaded (or changed by an action)
    UpdateConfig(LaunchpadConfig),
    /// Update the state of a container
    TaskDelta(TaskId, TaskDelta),
    /// Update the state of a wallet
    WalletDelta(WalletDelta),
}
```

The frontend has to deserialize events of the `Reaction` type
and update the `LaunchpadState`.

Components of the launchpad app have to listen to the changes
of the state and be re-rendered if the state is changed.

### Actions

To send actions like: transfer funds, save settings, etc. we
should send an event to the `tari://actions` stream of the
following `Action` type:

```rust
pub enum Action {
    Connect,
    SaveSettings(LaunchpadConfig),
    ///
    TransferFunds(Amount, Recipient),
    // etc.
}
```

The first event `Connect` is the special event that asks to re-send
the current state and should be sent when the main window is loaded
and the app is ready for processing events.

The event `SaveSettings` sends a new configuration to the engine when
the user changes it and saves it on the settings screen.

When the configuration is saved, the engine sends a delta
`UpdateConfig` which means the config was applied, and all affected
containers will be restarted.

To check whether the settings are changed or not, it's enough to compare
an instance from the state and the new configuration that exists in the settings screen.


### UI refactoring proposal

To use the new state, we should subscribe the frontend to events
from the `tari://actions` and provide that state using the **Redux**.
Ideally, the state should be available for all components, but
a rendering of them should depend on changes in specific state fields.

#### The cases

There are examples below how we could replaces some API calls
with that state.

##### Node Identity

###### Original

A node identity is necessary to start a wallet container. In the original
implementation we have to wait when the node container is started and
call the `node_identity` method.

```rust
pub async fn node_identity() -> Result<BaseNodeIdentity, String>
```

###### With the state

The node identity shouldn't be requested explicitly because the engine
gets it using the inner GRPC client as soon as the base node container
is available.

It's still possible to share it with the state, but the UI shouldn't
care about it for starting the wallet.

##### Syncing progress

###### Original

There is a method to subscribe to progress events from the base node:

```rust
pub async fn base_node_sync_progress(app: AppHandle<Wry>) -> Result<(), String>
```

The frontend used it to get progress during onboarding process.

###### With the state

The state has the field `containers` that keeps the `TaskStatus` objects:

```rust
pub struct TaskState {
    pub status: TaskStatus,
    pub tail: VecDeque<String>,
}
```

The task state contains the status of the container:

```rust
pub enum TaskStatus {
    Inactive,
    Pending,
    Progress(TaskProgress),
    Active,
}
```

And to get to know the container is ready or not (in progress) we should
get a value of the following field in the frontend:

```rust
state.containers["Base Node"].status
```

###### Original

We have a method to start a container:

##### Syncing progress

```rust
pub async fn start_service(
    app: AppHandle<Wry>,
    service_name: String,
    settings: ServiceSettings,
) -> Result<StartServiceResult, String>
```

But that problem the frontend also has to call that method
in due time: to start a wallet the base node has to be started
and synced.

###### With the state

With the state to start a specific container we shold mark is as started
in the configuration.

We could send `UpdateConfig` action and set a flag represented activity
of a service. For example:

```rust
config.wallet_active = true;
// then send the config to the backend here
```

We don't have to send the whole configuration and could add special
events that will mark a container as started and notify the frontend
about it.

But the state of that started/stopped container is always present
in the `LaunchpadState` and could be instantly rendered.
