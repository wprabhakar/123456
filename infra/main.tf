terraform {
  backend "s3" {
   bucket = "waltergun51-terraform-remote-state-one"
   key = "shorturl_state"
   region = "ap-southeast-1"
   dynamodb_table = "terraform_state"
  }
  required_providers {
   aws = {
     source = "hashicorp/aws"
   }
  }
}


provider "aws" {
  region = var.aws_region
  shared_credentials_files = ["$HOME/.aws/credentials"]
}
