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
> -p 5432:5432 \
> -e POSTGRES_USER=postgres \
> -e POSTGRES_PASSWORD=password \
> -e POSTGRES_DB=postgres \
> --name echo-server-database \
> -d postgres
> ```

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
  - [Swift](TODO)
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
4. Once you have constructed the payload you should send it to Echo Server, if
  you are using the hosted servers the following urls are to be used:
  - https://dev.echo.walletconnect.com (development)
  - https://staging.echo.walletconnect.com (staging)
  - https://echo.walletconnect.com (production)
  In the likely case you are using your own instance the base url will be up-to
  you to figure out but typically is `http://localhost:3000`

  The endpoint you're sending this request too will depend if you're targeting
  single-tenant or multi-tenant, all hosted servers run in multi-tenant mode and
  need a tenant created. Typically through the [cloud app](https://cloud.walletconnect.com)
  
  The single-tenant url is as follows: `<BASE_URL>/clients`.
  The multi-tenant url is as follows: `<BASE_URL>/<PROJECT_ID>/clients`.
  
  > **Note**
  > You must have registered your Project ID, see [here](#registering-tenant), if you're
  > using the hosted server the URL provided in the cloud app already has your
  > Project ID in it, so use the format of the single-tenant URL
5. Send a `POST` request with the JSON body from Step 3, to the URL from Step 4
  If everything is correct you should receive a `2XX` status code