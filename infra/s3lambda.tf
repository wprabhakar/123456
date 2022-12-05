resource "aws_s3_bucket" "lambda_bucket" {
 bucket = "shorurls-api-rust"
}

resource "aws_s3_bucket_acl" "bucket_acl" {
  bucket = aws_s3_bucket.lambda_bucket.id
  acl    = "private"
}
