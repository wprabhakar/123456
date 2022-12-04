terraform {
  backend "s3" {
  #  bucket = "waltergun51-terraform-remote-state-one"
   key = "shorturl"
   region = "ap-southeast-1"
  }
  required_providers {
   aws = {
     source = "hashicorp/aws"
   }
  }
}


provider "aws" {
  region = var.aws_region
  # shared_credentials_files = ["$HOME/.aws/credentials"]
}


variable "push_temp_bin_path" {
  description = "The binary path for the lambda."
  type = string
  default = "./bootstrap"
}


resource "aws_dynamodb_table" "terraform-lock" {
 name = "terraform_state"
 billing_mode = "PAY_PER_REQUEST"
#  read_capacity = 5
#  write_capacity = 5
 hash_key = "LockID"
 attribute {
   name = "LockID"
   type = "S"
 }
}

resource "aws_dynamodb_table" "shorturls_table" {
 name = "shorturls-table"
 billing_mode = "PAY_PER_REQUEST"
#  read_capacity= "1"
#  write_capacity= "1"
 attribute {
  name = "url"
  type = "S"
 }
 attribute {
  name = "slink"
  type = "S"
 }
 hash_key  = "url"
 range_key = "slink"
}

resource "aws_iam_role" "iam_for_lambda" {
 name = "iam_for_lambda"

 assume_role_policy = jsonencode({
   "Version" : "2012-10-17",
   "Statement" : [
     {
       "Effect" : "Allow",
       "Principal" : {
         "Service" : "lambda.amazonaws.com"
       },
       "Action" : "sts:AssumeRole"
     }
   ]
  })
}
          
resource "aws_iam_role_policy_attachment" "lambda_policy" {
   role = aws_iam_role.iam_for_lambda.name
   policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}
          
resource "aws_iam_role_policy" "dynamodb-lambda-policy" {
   name = "dynamodb_lambda_write_policy"
   role = aws_iam_role.iam_for_lambda.id
   policy = jsonencode({
      "Version" : "2012-10-17",
      "Statement" : [
        {
           "Effect" : "Allow",
           "Action" : ["dynamodb:*"],
           "Resource" : "${aws_dynamodb_table.shorturls_table.arn}"
        }
      ]
   })
}

