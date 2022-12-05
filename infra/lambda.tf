data "archive_file" "lambda_newlink_archive" {
  type = "zip"

  source_file = "${var.newlink_path}"   
  output_path = "bootstrap"
}

resource "aws_lambda_function" "newlink_lambda" {
  function_name = "newlink"
  memory_size = 128
  # source_code_hash = data.archive_file.lambda_newlink_archive.output_base64sha256
  # filename         = data.archive_file.lambda_newlink_archive.output_path

  handler = "bootstrap"
  runtime = "provided.al2"

  s3_bucket = aws_s3_bucket.lambda_bucket.id
  s3_key    = "newlink/bootstrap"

  role = aws_iam_role.iam_for_lambda.arn
}


# aws lambda create-function --function-name rustTest \
#   --handler bootstrap \
#   --zip-file fileb://./target/lambda/basic/bootstrap.zip \
#   --runtime provided.al2 \ # Change this to provided.al if you would like to use Amazon Linux 1.
#   --role arn:aws:iam::XXXXXXXXXXXXX:role/your_lambda_execution_role \
#   --environment Variables={RUST_BACKTRACE=1} \
#   --tracing-config Mode=Active
