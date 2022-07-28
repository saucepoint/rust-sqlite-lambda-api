## rust-sqlite-lambda-api

🦀 Create REST APIs with Rust & SQLite, hosted on AWS Lambda & EFS 🦀

### *Lightweight, fast, hyper-affordable, and preorganized*

The template, is an extension of [rust-lambda-api](https://github.com/saucepoint/rust-lambda-api). Like its predecessor, this repo includes structured & organized examples that are not found in *cargo-lambda* templates.

The file-based, cloud-hosted nature of SQLite introduces nuances around database schema migrations. This repo includes an additional Lambda function for managing the database

All of the AWS infrastructure is created & handled with Terraform

---

# Setup & Dependencies

1. [Configure AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-quickstart.html)
2. Install [Rust](https://www.rust-lang.org/tools/install) & [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)
    * `cargo install cargo-lambda`

2. [Install terraform](https://learn.hashicorp.com/tutorials/terraform/install-cli?in=terraform/aws-get-started)

2. Prepare an initial `.zip` for Terraform to upload to AWS
    ```bash
    cd lambda-api/
    cargo lambda build --output-format zip
    ```
2. Use terraform to spin up AWS infrastructure
    ```
    cd terraform/
    terraform init
    terraform apply
    // read the changes reported by terraform
    // type yes and hit Enter
    // Successful infra deployment will return:
    // Apply complete! Resources: 17 added, 0 changed, 0 destroyed.
    ```
    *Note:* The initial creation of the Lambda will take up to 1 min to process. If you attempt a `cargo lambda deploy` shortly after resource-creation, it will fail if the Lambda has an in-progress update

2. Continue the following steps to test the code:

2. Create the example sqlite table:
    ```bash
    cd db-migrations/

    # redeploy the function so it has the environment variable
    cargo lambda build --release && cargo lambda deploy --iam-role arn:aws:iam::<AWS_ACCOUNT_ID>:role/rust-sqlite-lambda-api --env-var MODE=prod

    # Invoke the lambda and provide the command which maps to a Rust function
    cargo lambda invoke --remote --data-ascii '{"command": "create_hello_table"}' db-migrations
    ```

2. Deploy the API so it's assigned a Function URL
    ```bash
    cd lambda-api
    cargo lambda build --release && cargo lambda deploy --iam-role arn:aws:iam::<AWS_ACCOUNT_ID>:role/rust-sqlite-lambda-api --enable-function-url --env-var MODE=prod
    ```

2. Test the API. Get the Function URL from the previous step, or on AWS console
    ```
    POST https://<BIG-ASS-HASH>.lambda-url.us-east-2.on.aws/hello
    // with raw JSON body:
    {
        "name": "saucepoint"
    }

    GET https://<BIG-ASS-HASH>.lambda-url.us-east-2.on.aws/hello?name=saucepoint
    // example response:
    {
        "id": 1,
        "name": "saucepoint",
        "count": 1
    }
    ```

# Configuration
### Adding Routes:

1. Create a new file such as `lambda-api/src/api/foo.rs`
    - Add GET/POST handlers, Request/Response structs, similar to [src/api/hello.rs](https://github.com/saucepoint/rust-sqlite-lambda-api/blob/44e0b6e5b5d3c41d78212bcac052cf81627da1c2/lambda-api/src/api/hello.rs)

2. Register the route in `lambda-api/src/main.rs` similar to [this](https://github.com/saucepoint/rust-sqlite-lambda-api/blob/44e0b6e5b5d3c41d78212bcac052cf81627da1c2/lambda-api/src/main.rs#L31-L38)
    ```rust
    "/foo" => {
        match method {
            Method::POST => api::foo::post(event, connection),
            Method::GET => api::foo::get(event, connection),
            _ => api::errors::handle_405(),
        }
    }
    ```

### Rename the function:

1. `lambda-api/Cargo.toml`
    - name = ~~"rust-sqlite-lambda-api"~~ --> NEW_FUNCTION_NAME
2. `terraform/main.tf`
    - function_name = ~~"rust-sqlite-lambda-api"~~ --> NEW_FUNCTION_NAME
    - filename = ~~"../lambda-api/target/lambda/rust-sqlite-lambda-api/bootstrap.zip"~~ --> "../lambda-api/target/lambda/NEW_FUNCTION_NAME/bootstrap.zip"

3. Apply changes
    ```bash
    cd lambda-api/
    cargo lambda build
    cd ../terraform/
    terraform apply
    ```

You can use terraform or AWS console to attach additional infrastructure

## Database Migrations

To apply changes to the sqlite database (i.e. creating tables, modifying tables, running data migrations, etc):

Please see: [db-migrations/README.md](db-migrations/README.md)

# Local Development

In a new terminal:
```bash
cd lambda-api/

cargo lambda watch
```

Testing:

The base URL will be *127.0.0.1:9000/lambda-url/lambda-api*
```
GET 127.0.0.1:9000/lambda-url/lambda-api/hello?name=saucepoint

POST 127.0.0.1:9000/lambda-url/lambda-api/hello
// with raw JSON body:
{
    "name": "saucepoint"
}
```

*Note*: when deployed to Lambda, your route will not have `/lambda-url/lambda-api` prefix:

*https://RANDOM_HASH.lambda-url.REGION.on.aws/hello*


# Deploying

Please see [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) for additional flags (such as environment variables)

This is my preferred deployment call:

*Get your IAM Role's ARN from the AWS web console*
```bash
cargo lambda build --release && cargo lambda deploy --enable-function-url --iam-role arn:aws:iam::<AWS_ACCOUNT_NUMBER>:role/rust-lambda-api --env-var MODE=prod
```

---

### *Disclaimer*
This template repo is intended for hobbyists and experiments. The following bits will need to be modified & enhanced before it can be considered production-ready:

🚩 Tests - this repo totally lacks automated tests

🚩  Modularized Terraform - the current files will collide with resources of the same name. The files should be better organized & more configurable such that it can tap into existing infrastructure (VPCs)

🚩  AWS API Gateway Trigger - Typical Lambda REST APIs sit behind API Gateway. It didn't stop me from getting this prototype functional, so I didn't really bother

🚩  API authentication - this repo offers no examples around auth-required APIs

Feel free to submit PRs!

---

I'm tinkering outside of my 9-5, with plans to launch *something, eventually*

Find me on twitter [@saucepoint](https://twitter.com/saucepoint)
