# Echo Server Contribution Guide
This guide helps contributors & WalletConnect team members work on Echo Server,
hopefully with ease, there are sections for most actions you may want to take as
well as a full setup guide.

- [Initial Setup](#initial-setup)
- [Testing with the Relay](#testing-with-the-relay)
- [Client Registration](#client-registration)
- [Sending a Push Notification](#sending-a-push-notification)
- [Registering a Tenant](#registering-tenant)
- [Updating a Tenant's Provider Info](#updating-tenant)

# Initial Setup

1. Clone the repository
```
git clone https://github.com/walletconnect/echo-server.git
```
2. Setup environment variables, see [`.env.example`](./.env.example) for all
   the example environment variables. For specific example see:
   - [`.env.single-tenant-example`](./.env.single-tenant-example)
   - [`.env.multi-tenant-example`](./.env.multi-tenant-example)
3. Run using `cargo run`

> **Note**
> In single-tenant mode you can only register clients where you have set
> the required environment variables. In multi-tenant mode, you must register
> a tenant, see [here](#registering-tenants)

> **Warning**
> Echo Server requires a postgres database, the `DATABASE_URL` & `TENANT_DATABASE_URL`
> should both be valid postgres database url. We **recommend** using a clean database
> for both. An example docker command is below for how to start a container running postgres
> ```bash
> docker run \
>   -p 5432:5432 \
>   -e POSTGRES_USER=postgres \
>   -e POSTGRES_PASSWORD=password \
>   -e POSTGRES_DB=postgres \
>   --name echo-server-database \
>   -d postgres
> ```

## Validation
To test your instance is working, open a browser and visit the `/health` endpoint.

You could also, optionally, run the integration tests against your instance by
following these steps:
1. Open a terminal in the same directory as the code
2. Run `yarn install`
3. Run `yarn integration:dev`

> **Note**
> The integration tests presume you are running your local instance on 
> port `3000`

# Testing with the Relay
To test with the relay your local instance needs to be available on the internet.
To do this you can use a tunnel, e.g. ngrok or cloudflare tunnels. Once you have a
public url that points to your local instance. Go into the [cloud app](https://cloud.walletconnect.com),
find your project and put that url as the `push_url` in the settings. Then the relay
will be sending the notifications to your local instance.

> **Warning**
> The Relay caches the `push_url` field, it can take up-to 5 minutes

# Figuring out what URL to use
This section is referred to by a few others throughout the guide, it helps you
figure out what URL you should use as the base for your requests.

If are using the hosted servers the following urls are to be used:
- https://dev.echo.walletconnect.com (development)
- https://staging.echo.walletconnect.com (staging)
- https://echo.walletconnect.com (production)

In the likely case you are using your own instance the base url will be up-to
you to figure out but typically is `http://localhost:3000`

The endpoint you're sending this request too will depend if you're targeting
single-tenant or multi-tenant, all hosted servers run in multi-tenant mode and
need a tenant created. Typically through the [cloud app](https://cloud.walletconnect.com)

The single-tenant url is just the Public url, as shown here: `<BASE_URL>`.

The multi-tenant url appends the Project ID to the Public URL 
as shown here: `<BASE_URL>/<PROJECT_ID>`.

> **Note**
> You must have registered your Project ID, see [here](#registering-tenant), if you're
> using the hosted server the URL provided in the cloud app already has your
> Project ID in it, so use the format of the single-tenant URL

# Client Registration

> **Note**
> This section requires you have your project setup as a prerequisite, see
> [here](#initial-setup)

To receive notifications you need to have registered you client id, from the
Swift/JS/Kotlin/etc SDK with Echo Server and paired it with a push type (provider)
as well as the respective push token. 

This method is the same for both multi-tenant and single-tenant mode, the only
difference is the URL you are sending your requests to.

> **Warning**
> These steps are only if you want to do the client registration manually, if
> you wish to use an SDK please look at the specific docs [here](https://docs.walletconnect.com/2.0/api/push)

1. Get your Client ID, see below for specific functions in each SDK
  - [Swift](https://docs.walletconnect.com/2.0/swift/echo/usage#configure-client)
  - [Kotlin](TODO)
  - [JS](https://github.com/WalletConnect/walletconnect-monorepo/blob/v2.0/packages/core/src/controllers/crypto.ts#L51)
2. Get your push token from the provider, each provider has their own specific
   docs so please refer to them
3. Construct the payload to be sent to Echo Server for client registration, see
   below for how this json payload should look.
```jsonc
{
    "client_id": "<CLIENT_ID>", // The Client ID from Step 1
    "type": "<TYPE>", // See note below
    "token": "<DEVICE_TOKEN>" // The token from Step 2
}
```
> **Note**
> The `type` should be the push provider you got the token from in Step 2,
> this should be one of the following enum values:
> - `fcm`: Firebase Cloud Messaging
> - `apns`: Apple Push Notification Service (production)
> - `apns-sandbox`: Apple Push Notification Service (sandbox)
4. Once you have constructed the payload you should send it to Echo Server, to
  find out what URL you should use as the base see [here](#figuring-out-what-url-to-use).

  Append `/clients` to your base url and you have the URL you should send the 
  request to, below are some examples of contructed URLs to help
  ```
  https://echo.walletconnect.com/83f11e753439fab08222b45e2d029eab/clients
  http://localhost:3000/clients
  http://localhost:3000/83f11e753439fab08222b45e2d029eab/clients
  ```
5. Send a `POST` request with the JSON body from Step 3, to the URL from Step 4
  If everything is correct you should receive a `2XX` status code

# Sending a Push Notification

> **Note**
> The sections makes a few assumptions:
> - You have setup a local instance and have disabled signature validation
> - You have registered a client, following the steps [here](#client-registration)

Echo Server supports 2 kinds of notifications, they are typically delivered by
the Relay. The Relay can send "plain text" notifications which include the notification
in the JSON payload or the notification can be "encrypted" which means the JSON-RPC
request is just forwarded to the SDKs on the device to be decrypted - using
`mutable-content` with APNS & the `data` object for FCM.

This guide can only show an example for sending Plain Text notifications, if you
want to test Encrypted notifications please see the [Testing with the Relay section](#testing-with-the-relay)
which explains how to have the relay send notifications to your local instance.

1. Get your Base URL, as explained [here](#figuring-out-what-url-to-use).
2. Get the Client ID you want to send the notification to, see below for 
   specific functions in each SDK:
   - [Swift](https://docs.walletconnect.com/2.0/swift/echo/usage#configure-client)
   - [Kotlin](TODO)
   - [JS](https://github.com/WalletConnect/walletconnect-monorepo/blob/v2.0/packages/core/src/controllers/crypto.ts#L51)
3. Construct the URL you will send the request to, this is done in the following
  format: `<BASE_URL>/clients/<CLIENT_ID>`. Here are some examples:
  ```
  https://echo.walletconnect.com/83f11e753439fab08222b45e2d029eab/clients/z6MkusgKjrfmeboSVKLhf6fpGnToPsvA2yJb7wJuVbL3Jeig
  http://localhost:3000/clients/z6MkusgKjrfmeboSVKLhf6fpGnToPsvA2yJb7wJuVbL3Jeig
  http://localhost:3000/83f11e753439fab08222b45e2d029eab/clients/z6MkusgKjrfmeboSVKLhf6fpGnToPsvA2yJb7wJuVbL3Jeig
  ```
4. Create a unique id for this push notification, Echo Server uses this ID to
  prevent duplicate notifications being sent to devices. This can be any string
  value but the Relay defaults to UUID. You can generate a UUID many ways be here
  are some websites that have UUID generation:
  - https://www.uuidgenerator.net/
  - [Cyber Chef](https://gchq.github.io/CyberChef/#recipe=Generate_UUID())
5. Construct the "blob", in this instance the blob is a base64 encoded JSON string
  which replicates a Push API message. Echo Server will decode and parse the blob
  before sending a notification to the client. See below for the format of the JSON
  and a command to serialise and encode.
  ```jsonc
{
  "title": "Example Notification",
  "body": "This is an example notification"
}
  ```
  > **Note**
  > The full payload is available in the specs [here](https://docs.walletconnect.com/2.0/specs/clients/push/data-structures#push-message)
  
  To then serialise and encode this json, you should do something similar to the below
  command
  ```bash
  echo "{\"title\": \"Example Notification\",\"body\": \"This is an example notification\"}" | base64
  ```
6. Construct the JSON body for the HTTP. This will use the ID from Step 4, and the
  Blob from Step 5. You should create the payload in the following format:
  ```jsonc
    {
        "id": "<unique_identifier>", // value from Step 4
        "payload": {
          "topic": undefined,
          "flags": 0,
          "blob": "<blob>" // value from Step 5
        }
    }
  ```

  > **Note**
  > The topic is only used for encrypted notifications.
  > 
  > Echo Server figures out what kind of notification is being sent by using
  > the flags property which is a Bitmasked integer, see the [spec](./spec/spec.md#send-notification)
  > for more info.
7. Send a `POST` request to your URL from Step 3, with the JSON body from Step 6.
  If everything is successful you should get a `2XX` response and see a push notification
  on your device.