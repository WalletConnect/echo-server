# Echo Server
Push server for the WalletConnect v2 Protocol

> **Note** This is only available for WalletConnect v2 and is a breaking change from the Push supported in v1.

## Notification Providers
This list contains both supported and potentially planned providers
- [x] FCM (API Key)
- [ ] FCM (Google Services)
- [x] APNS (Certificate Based)
- [ ] APNS (Token Based)
- [ ] Web Push

## Supporting Notifications
> **Note** Full documentation will be available soon. This is only a brief overview.

There are 3 options for receiving notifications within your wallet:
1. Use the hosted platform. (available in the [cloud app](https://cloud.walletconnect.com) soon)
2. Host this rust implementation - there is an included [`terraform`](https://github.com/WalletConnect/echo-server/tree/main/terraform) 
configuration to help with this.
3. Write your own implementation using the [spec](./spec/spec.md).

When using the hosted platform or self-hosting this implementation you have to provide the instance
you FCM API Key or APNS certificates and then - following the FCM/APNS docs - add support for that within your
wallet.

You also have to register the device with the instance of Echo Server once when the client_id is initially
generated. By sending a POST request to `<INSTANCE_URL>/clients` as per the [spec](./spec/spec.md).

## Multi-tenancy
Echo Server supports multi-tenancy. However, the management of tenants is delegated to an alternative service. 
To enable multi-tenancy you need to specify a `TENANT_DATABASE_URL` which will then disable the single-tenant
endpoints in favour of endpoints with a `/:tenant_id` prefix e.g. `/:tenant_id/client/:id`

## Running locally

```
# Run a postgres db for functional tests
# This will be removed in future revisions
# such that you can run functional tests without
# any prerequisites
docker run -p 5432:5432 --name some-postgres2 -e POSTGRES_HOST_AUTH_METHOD=trust -d postgres
cargo test
```

## Running tests locally

```
yarn install
yarn integration:dev # or yarn integration:staging
```

## Deploying infrastructure

```
terraform -chdir=terraform init
terraform -chdir=terraform workspace select dev/staging/prod
terraform -chdir=terraform apply  -var-file="vars/$(terraform -chdir=terraform workspace show).tfvars"
```

## Contact
If you wish to integrate Push functionality into your Wallet (only available on v2), please contact us.

## Contributing
To get started with contributing to Echo Server, look at the [open issues](https://github.com/WalletConnect/echo-server/issues?q=is:issue+is:open+label:%22help+wanted%22).
New contributors can also look at the [issues labeled with "good first issue"](https://github.com/WalletConnect/echo-server/issues?q=is:issue+is:open+label:%22good+first+issue%22) 
as they should be suitable to people who are looking at the project for the first time.

## License
Copyright 2022 WalletConnect, Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
