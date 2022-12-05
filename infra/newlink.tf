resource "aws_apigatewayv2_integration" "newlink_integration" {
  api_id           = aws_apigatewayv2_api.shorturl_api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Get shortlink for URL"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.newlink_lambda.invoke_arn

  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "newlink_route" {
  api_id    = aws_apigatewayv2_api.shorturl_api.id
  route_key = "POST /newlink"
  target    = "integrations/${aws_apigatewayv2_integration.newlink_integration.id}"
}


resource "aws_lambda_permission" "newlink_api_permission" {
  function_name = aws_lambda_function.newlink_lambda.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.shorturl_api.execution_arn}/*/*/${split("/", aws_apigatewayv2_route.newlink_route.route_key)[1]}"
}

resource "aws_s3_object" "newlink_folder" {
  bucket = aws_s3_bucket.lambda_bucket.id
  key    = "newlink/"
  # content_type = "application/x-directory"
}

# resource "aws_s3_bucket_object" "newlink_folder" {
#     bucket = aws_s3_bucket.lambda_bucket.id
#     key    = "newlink/"
#     content_type = "application/x-directory"
# }
