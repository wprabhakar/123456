resource "aws_apigatewayv2_integration" "getlink_integration" {
  api_id           = aws_apigatewayv2_api.shorturl_api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get shortlink for URL"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.getlink_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "getlink_route" {
  api_id    = aws_apigatewayv2_api.shorturl_api.id
  route_key = "GET /{id}"
  target    = "integrations/${aws_apigatewayv2_integration.getlink_integration.id}"
}


resource "aws_lambda_permission" "getlink_api_permission" {
  function_name = aws_lambda_function.getlink_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.shorturl_api.execution_arn}/*/*/${split("/", aws_apigatewayv2_route.getlink_route.route_key)[1]}"
}

resource "aws_s3_object" "getlink_folder" {
  bucket = aws_s3_bucket.lambda_bucket.id
  key    = "getlink/"
}

data "aws_s3_object" "getlink_sha256" {
  bucket = aws_s3_bucket.lambda_bucket.id
  key    = "getlink/bootstrap"
#  etag     = filemd5("bootstrap")
  # source_hash = filebase64sha256("getlink/bootstrap")
}

resource "aws_lambda_function" "getlink_lambda" {
  depends_on = [
    aws_s3_object.getlink_folder,
    data.aws_s3_object.newlink_sha256
  ]
  function_name = "getlink"
  memory_size = 128

  source_code_hash = "${data.aws_s3_object.getlink_sha256.etag}"
#  s3_object_version = data.aws_s3_object.getlink_sha256.version_id
  handler = "bootstrap"
  runtime = "provided.al2"

  s3_bucket = aws_s3_bucket.lambda_bucket.id
  s3_key    = "getlink/bootstrap"

  role = aws_iam_role.iam_for_lambda.arn
}
