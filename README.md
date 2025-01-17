# Lambda Deployment CLI for CI/CD

A CLI tool for deploying AWS Lambda functions.

Designed to streamline the deployment process in any CI/CD environment with efficiency and reliability.

## AWS APIs Used

This tool interacts with the following AWS APIs:

- [`UpdateFunctionCode`](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateFunctionCode.htm): Updates the code of an existing Lambda function.
- [`PublishVersion`](https://docs.aws.amazon.com/lambda/latest/api/API_PublishVersion.html): Publishes a new version of a Lambda function.
- [`UpdateAlias`](https://docs.aws.amazon.com/lambda/latest/api/API_UpdateAlias.html): Updates the alias of a Lambda function to point to a specific version.  

These APIs require appropriate IAM permissions to function correctly.