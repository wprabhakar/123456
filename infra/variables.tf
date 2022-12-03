# variables.tf

# service name - this will be used as a prefix for most of the resource names
variable "service_name" {
  type = string
  default = "shorturls-api"
}

# AWS region
variable "region" {
  type = string
  default = "ap-southeast-1"
}

