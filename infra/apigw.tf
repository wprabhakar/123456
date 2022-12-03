resource "aws_apigatewayv2_api" "shorturl_api" {
  name          = "API"
  description   = "ShortUrls API"
  protocol_type = "HTTP"
}


resource "aws_apigatewayv2_stage" "api_stage" {
  api_id      = aws_apigatewayv2_api.shorturl_api.id
  name        = "$default"
  auto_deploy = true
}
