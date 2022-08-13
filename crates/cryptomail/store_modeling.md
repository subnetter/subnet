# Data Modeling

## Requirements - Alpha Version


New thread flow:

2. User creates random thread ida message, payment with thread (blockchain tx) id and sends it to server. Deposit is with same thread id
3. Server ensures submitted thread id is unique (no existing thread keyed by this id in stored) and creates a new thread. No need for thread pool - thread creation is serialized via actor system service in the server :-)

- Threads are only between 2 parties.
- Threads ids must be unique per global namespace.
- Account names must be unique per global namespace.
- Message ids are unique per thread namespace

- Messages persisted using key `thread_id || message_id` key in store in `messages column space`.
- Threads hold an ordered list of `message_ids`.
- Threads should be stored using `thread_id` in `threads column space`.
- Accounts should be indexed by `account_address` in `accounts column space`.  
- Unique account names should be stored keyed by `name` in the `account_names column space`.
- Users thread boxes should be indexed in `boxes space` by `[account_address]_[Inbox | Sent | Arvice]` key.

- New generated threadId should be saved in `thread_id` in 'thread_ids_pool' column space.
- After user creates a thread with thread_id -> remove it from pool.

- Server stores 3 thread boxes for each user:
    - `Inbox` contains all threads started by others with user as recipient or threads user started by sending a message to another user.
    - `Sent` includes all threads that the user has sent a message in (new one or reply).
    - `Archive` contains all threads moved by user from Inbox.

- A user can delete a thread but another user may still have this thread. So a thread can be in 0 to 2 boxes.

## Store Design
- Each message should be stored once and not once per a thread box containing it.
- K,V store such as rocks - no queries are needed - all data is indexed.
- Need some atomicity around unique thread_ids and names in global namespace to ensure uniqueness.
- Thread-boxes hold ordered sets of message ids. Users inbox is already sorted according to its client display order.
- No need for pagination yet in api - maybe if service becomes popular. 
- Messages stored in k,v store with unique key: `thread_id || message_id`.

- maintain column family for `public_directory` - (public_account_address, data: ????)

## Store Types Definitions
Used as store keys and values.

- `AccountPublicKey` - user generated from keypair. For ed25519 keys. 32 bytes.
- `AccountName` - globally unique utf-8 string.
- `ThreadBoxType` - { Inbox | Sent | Archive | custom (in the future) }
- `Thread` - an ordered list of `MessageIds` 
- `Box` - ordered list of `ThreadIds`
- `UnconfirmedMessages` - set of `MessageIds`
- `AccountAddress` - last 20 bytes of AccountPublicKey.
- `ThreadId` - globally unique random u64, client generated, check for uniqueness on server before adding to system.
- `MessageThreadId` - a unique u32 id of a message in a thread.
- `MessageId` - { `ThreadId` || `MessageThreadId` }
- `ThreadBoxId` - { `AccountAddress` || `ThreadBoxType` }
- `BoxId` - { `AccountAddress` || `ThreadBoxType` }


## Database Column Families Definitions

### THREADS
k,v := (`ThreadId`, `Thread)

### ACCOUNTS
k,v := (`AccountAddress, `Account`)

### Messages
k,v := (`MessageId`, `Message`)

### PUB_ACCOUNTS
k,v := (`AccountName`, `AccountAddress`)

### ACCOUNT_NAMES
k,v := (`AccountName`, `AccountAddress`)

### BOXES
k,v := (`BoxId`, `Box`)

### SYSTEM
k,v := (`UNCONFIRMED_MSGS_KEY`, `UnconfirmedMessages`)








