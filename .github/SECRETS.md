## `PROD_JWT_SECRET` & `STAGING_JWT_SECRET`

From 1Password: `cloud/push-server-jwt/prod` and `cloud/push-server-jwt/staging`

Generated randomly and used by Cloud app to sign JWTs.

## `ECHO_TEST_FCM_V1_CREDENTIALS`

FCM v1 service account credentials for test cases.

Setup:
- Go to the Push Server Tests Firebase project: https://console.firebase.google.com/project/push-server-tests-cc0f7/settings/cloudmessaging
- On Cloud Messaging tab, under the "Firebase Cloud Messaging API (V1)" header, click the "Manage Service Accounts" link
- Select the service account and click "Manage keys"
- Click "Add key" and select "Create new key" and pick JSON
