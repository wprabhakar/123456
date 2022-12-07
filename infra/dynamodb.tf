
resource "aws_dynamodb_table" "terraform-lock" {
 name = "terraform_state"
 billing_mode = "PAY_PER_REQUEST"
 hash_key = "LockID"
 attribute {
   name = "LockID"
   type = "S"
 }
}

resource "aws_dynamodb_table" "shorturls_table" {
 name = "shorturls-table"
#  billing_mode = "PAY_PER_REQUEST"
 billing_mode   = "PROVISIONED"
 read_capacity  = 1
 write_capacity = 1
 attribute {
  name = "url"
  type = "S"
 }
 attribute {
  name = "slink"
  type = "S"
 }
 hash_key  = "url"
 global_secondary_index {
    name               = "slink_gsi"
    hash_key           = "slink"
    read_capacity      = 100
    write_capacity     = 1
    projection_type    = "ALL"
  }
}
