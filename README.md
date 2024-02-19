# self-sensored-io
WHERE I LEFT OFF:
1. Map activity to item. Maybe move to .save() method on Activity
2. Create a base API Gateway Proxy Request struct. Figure out how to make it is DRY as possible.
3. The `put` endpoint needs to check if the item exists, and update it, otherwise, save it.

# DynamoDB Rust Operations
https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/operation/index.html

# Run Command
```
AWS_PROFILE=personal TABLE=house_codex cargo lambda watch -a 127.0.0.1 -p 9001
```


## Impediments
```
Error: Unable to upload artifact PutFunction referenced by CodeUri parameter of PutFunction resource.
An HTTP Client raised an unhandled exception: sequence item 0: expected str instance, bytes found
```
Solved by:
https://github.com/aws/aws-sam-cli/issues/6667#issuecomment-1939435951
https://github.com/aws/aws-sam-cli/issues/6668

## Setting up SAM CLI on MacOS

The Python version finally worked for me.
https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/manage-sam-cli-versions.html#manage-sam-cli-versions-install-virtual