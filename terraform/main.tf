terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region  = "us-east-2"
}

# -----------------------------------------------------------------------------
# IAM Role for the Lambda Function
# -----------------------------------------------------------------------------
resource "aws_iam_role" "rust-sqlite-lambda-api" {
  name = "rust-sqlite-lambda-api"
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_iam_policy_attachment" "attach-basiclambda" {
  name = "attach-basiclambda"
  roles = ["${aws_iam_role.rust-sqlite-lambda-api.name}"]
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_policy_attachment" "attach-lambdavpc" {
  name = "attach-lambdavpc"
  roles = ["${aws_iam_role.iam_for_lambda.name}"]
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
}


# ------------------------------------------------------------
# VPC, where the Lambda and EFS will exist
# ------------------------------------------------------------
resource "aws_vpc" "rust-sqlite-lambda-vpc" {
  cidr_block = "10.10.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support = true
}

resource "aws_subnet" "lambda_subnet" {
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id
  cidr_block = "10.10.102.0/24"
}

resource "aws_subnet" "lambda_public_subnet" {
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id
  cidr_block = "10.10.101.0/24"
}

resource "aws_eip" "lambda_eip" {
  vpc = true
}

resource "aws_internet_gateway" "lambda_igw" {
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id
}

resource "aws_nat_gateway" "lambda_nat_gateway" {
  subnet_id = aws_subnet.lambda_public_subnet.id
  allocation_id = aws_eip.lambda_eip.id
}

resource "aws_route_table" "lambda_public_route_table" {
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id
  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.lambda_igw.id
  }

  depends_on = [
    aws_internet_gateway.lambda_igw
  ]
}

resource "aws_route_table" "lambda_private_route_table" {
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    nat_gateway_id = aws_nat_gateway.lambda_nat_gateway.id
  }

  depends_on = [
    aws_nat_gateway.lambda_nat_gateway
  ]
}

resource "aws_security_group" "rust-sqlite-lambda-sg" {
  name = "rust-sqlite-lambda-sg"
  vpc_id = aws_vpc.rust-sqlite-lambda-vpc.id
  description = "lambda security group"
  ingress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
  }

  egress {
    from_port        = 0
    to_port          = 0
    protocol         = "-1"
    cidr_blocks      = ["0.0.0.0/0"]
  }
  depends_on = [
    aws_vpc.rust-sqlite-lambda-vpc
  ]
}


# ------------------------------------------------------------
# Lambda Functions
# ------------------------------------------------------------
resource "aws_lambda_function" "sqlite-lambda-api" {
  function_name = "rust-sqlite-lambda-api"
  role = aws_iam_role.rust-sqlite-lambda-api.arn
  handler = "bootstrap"
  runtime = "provided.al2"
  filename = "../lambda-api/target/lambda/rust-sqlite-lambda-api/bootstrap.zip"
  timeout = 30  # 30 second timeout for Lambda invokations
}

resource "aws_lambda_function" "db-migrations" {
  function_name = "db-migrations"
  role = aws_iam_role.iam_for_lambda.arn
  handler = "bootstrap"
  runtime = "provided.al2"
  filename = "../db-migrations/target/lambda/db-migrations/bootstrap.zip"
  timeout = 120  # 120 second timeout for db migrations
  
  file_system_config {
    arn = aws_efs_access_point.lambda_efs_access_point.arn
    local_mount_path = "/mnt/efs"
  }

  vpc_config {
    subnet_ids = [aws_subnet.lambda_subnet.id, aws_subnet.lambda_public_subnet.id]
    security_group_ids = [aws_security_group.rust-sqlite-lambda-sg.id]
  }

  depends_on = [aws_efs_mount_target.lambda_mount_target]
}


# ------------------------------------------------------------
# EFS
# ------------------------------------------------------------
# EFS resource that will attached to the function
resource "aws_efs_file_system" "lambda_efs" {
}

# mount target to connect the EFS to the subnet
resource "aws_efs_mount_target" "lambda_mount_target" {
  file_system_id = aws_efs_file_system.lambda_efs.id
  subnet_id      = aws_subnet.lambda_subnet.id
  security_groups = [aws_security_group.rust-sqlite-lambda-sg.id]
}

# EFS access point used by the lambda function
resource "aws_efs_access_point" "lambda_efs_access_point" {
  file_system_id = aws_efs_file_system.lambda_efs.id
  root_directory {
    path = "/lambda"
    creation_info {
      owner_gid   = 1000
      owner_uid   = 1000
      permissions = "777"
    }
  }

  posix_user {
    gid = 1000
    uid = 1000
  }
}