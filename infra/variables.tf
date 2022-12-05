# variables.tf

# service name - this will be used as a prefix for most of the resource names
variable "service_name" {
  type = string
  default = "shorturls-api"
}

# AWS region
variable "aws_region" {
  type = string
  default = "ap-southeast-1"
}

variable "fetchurl_path" {
  description = "The path for the fetch lambda."

  type    = string
  default = "./shorturl/bootstrap"
}

# variable "newlink_path" {
#   description = "The path for the newlink lambda."

#   type    = string
#   default = "newlink/bootstrap"
#   # default = "../newlink/target/lambda/bootstrap/bootstrap"
# }
