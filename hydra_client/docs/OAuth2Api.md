# \OAuth2Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_o_auth2_consent_request**](OAuth2Api.md#accept_o_auth2_consent_request) | **Patch** /oauth2/consent/requests/{id}/accept | Accept a consent request
[**create_o_auth2_client**](OAuth2Api.md#create_o_auth2_client) | **Post** /clients | Create an OAuth 2.0 client
[**delete_o_auth2_client**](OAuth2Api.md#delete_o_auth2_client) | **Delete** /clients/{id} | Deletes an OAuth 2.0 Client
[**flush_inactive_o_auth2_tokens**](OAuth2Api.md#flush_inactive_o_auth2_tokens) | **Post** /oauth2/flush | Flush Expired OAuth2 Access Tokens
[**get_o_auth2_client**](OAuth2Api.md#get_o_auth2_client) | **Get** /clients/{id} | Get an OAuth 2.0 Client.
[**get_o_auth2_consent_request**](OAuth2Api.md#get_o_auth2_consent_request) | **Get** /oauth2/consent/requests/{id} | Receive consent request information
[**get_well_known**](OAuth2Api.md#get_well_known) | **Get** /.well-known/openid-configuration | Server well known configuration
[**introspect_o_auth2_token**](OAuth2Api.md#introspect_o_auth2_token) | **Post** /oauth2/introspect | Introspect OAuth2 tokens
[**list_o_auth2_clients**](OAuth2Api.md#list_o_auth2_clients) | **Get** /clients | List OAuth 2.0 Clients
[**oauth_auth**](OAuth2Api.md#oauth_auth) | **Get** /oauth2/auth | The OAuth 2.0 authorize endpoint
[**oauth_token**](OAuth2Api.md#oauth_token) | **Post** /oauth2/token | The OAuth 2.0 token endpoint
[**reject_o_auth2_consent_request**](OAuth2Api.md#reject_o_auth2_consent_request) | **Patch** /oauth2/consent/requests/{id}/reject | Reject a consent request
[**revoke_o_auth2_token**](OAuth2Api.md#revoke_o_auth2_token) | **Post** /oauth2/revoke | Revoke OAuth2 tokens
[**update_o_auth2_client**](OAuth2Api.md#update_o_auth2_client) | **Put** /clients/{id} | Update an OAuth 2.0 Client
[**userinfo**](OAuth2Api.md#userinfo) | **Post** /userinfo | OpenID Connect Userinfo
[**well_known**](OAuth2Api.md#well_known) | **Get** /.well-known/jwks.json | Get Well-Known JSON Web Keys


# **accept_o_auth2_consent_request**
> accept_o_auth2_consent_request(ctx, id, consent_request_acceptance)
Accept a consent request

Call this endpoint to accept a consent request. This usually happens when a user agrees to give access rights to an application.   The consent request id is usually transmitted via the URL query `consent`. For example: `http://consent-app.mydomain.com/?consent=1234abcd`   The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:oauth2:consent:requests:<request-id>\"], \"actions\": [\"accept\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
  **consent_request_acceptance** | [**ConsentRequestAcceptance**](ConsentRequestAcceptance.md)|  | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_o_auth2_client**
> ::models::OAuth2Client create_o_auth2_client(ctx, o_auth2_client)
Create an OAuth 2.0 client

Create a new OAuth 2.0 client If you pass `client_secret` the secret will be used, otherwise a random secret will be generated. The secret will be returned in the response and you will not be able to retrieve it later on. Write the secret down and keep it somwhere safe.  OAuth 2.0 clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities. To manage ORY Hydra, you will need an OAuth 2.0 Client as well. Make sure that this endpoint is well protected and only callable by first-party components.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:clients\"], \"actions\": [\"create\"], \"effect\": \"allow\" } ```  Additionally, the context key \"owner\" is set to the owner of the client, allowing policies such as:  ``` { \"resources\": [\"rn:hydra:clients\"], \"actions\": [\"create\"], \"effect\": \"allow\", \"conditions\": { \"owner\": { \"type\": \"EqualsSubjectCondition\" } } } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **o_auth2_client** | [**OAuth2Client**](OAuth2Client.md)|  | 

### Return type

[**::models::OAuth2Client**](oAuth2Client.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_o_auth2_client**
> delete_o_auth2_client(ctx, id)
Deletes an OAuth 2.0 Client

Delete an existing OAuth 2.0 Client by its ID.  OAuth 2.0 clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities. To manage ORY Hydra, you will need an OAuth 2.0 Client as well. Make sure that this endpoint is well protected and only callable by first-party components.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:clients:<some-id>\"], \"actions\": [\"delete\"], \"effect\": \"allow\" } ```  Additionally, the context key \"owner\" is set to the owner of the client, allowing policies such as:  ``` { \"resources\": [\"rn:hydra:clients:<some-id>\"], \"actions\": [\"delete\"], \"effect\": \"allow\", \"conditions\": { \"owner\": { \"type\": \"EqualsSubjectCondition\" } } } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the OAuth 2.0 Client. | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **flush_inactive_o_auth2_tokens**
> flush_inactive_o_auth2_tokens(ctx, ctx, optional)
Flush Expired OAuth2 Access Tokens

This endpoint flushes expired OAuth2 access tokens from the database. You can set a time after which no tokens will be not be touched, in case you want to keep recent tokens for auditing. Refresh tokens can not be flushed as they are deleted automatically when performing the refresh flow.   ``` { \"resources\": [\"rn:hydra:oauth2:tokens\"], \"actions\": [\"flush\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **flush_inactive_o_auth2_tokens_request** | [**FlushInactiveOAuth2TokensRequest**](FlushInactiveOAuth2TokensRequest.md)|  | 

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_o_auth2_client**
> ::models::OAuth2Client get_o_auth2_client(ctx, id)
Get an OAuth 2.0 Client.

Get an OAUth 2.0 client by its ID. This endpoint never returns passwords.  OAuth 2.0 clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities. To manage ORY Hydra, you will need an OAuth 2.0 Client as well. Make sure that this endpoint is well protected and only callable by first-party components.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:clients:<some-id>\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```  Additionally, the context key \"owner\" is set to the owner of the client, allowing policies such as:  ``` { \"resources\": [\"rn:hydra:clients:<some-id>\"], \"actions\": [\"get\"], \"effect\": \"allow\", \"conditions\": { \"owner\": { \"type\": \"EqualsSubjectCondition\" } } } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the OAuth 2.0 Client. | 

### Return type

[**::models::OAuth2Client**](oAuth2Client.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_o_auth2_consent_request**
> ::models::OAuth2ConsentRequest get_o_auth2_consent_request(ctx, id)
Receive consent request information

Call this endpoint to receive information on consent requests. The consent request id is usually transmitted via the URL query `consent`. For example: `http://consent-app.mydomain.com/?consent=1234abcd`   The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:oauth2:consent:requests:<request-id>\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the OAuth 2.0 Consent Request. | 

### Return type

[**::models::OAuth2ConsentRequest**](oAuth2ConsentRequest.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_well_known**
> ::models::WellKnown get_well_known()
Server well known configuration

The well known endpoint an be used to retrieve information for OpenID Connect clients. We encourage you to not roll your own OpenID Connect client but to use an OpenID Connect client library instead. You can learn more on this flow at https://openid.net/specs/openid-connect-discovery-1_0.html

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::WellKnown**](wellKnown.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **introspect_o_auth2_token**
> ::models::OAuth2TokenIntrospection introspect_o_auth2_token(ctx, ctx, token, optional)
Introspect OAuth2 tokens

The introspection endpoint allows to check if a token (both refresh and access) is active or not. An active token is neither expired nor revoked. If a token is active, additional information on the token will be included. You can set additional data for a token by setting `accessTokenExtra` during the consent flow.  ``` { \"resources\": [\"rn:hydra:oauth2:tokens\"], \"actions\": [\"introspect\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| The string value of the token. For access tokens, this is the \\\&quot;access_token\\\&quot; value returned from the token endpoint defined in OAuth 2.0 [RFC6749], Section 5.1. This endpoint DOES NOT accept refresh tokens for validation. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| The string value of the token. For access tokens, this is the \\\&quot;access_token\\\&quot; value returned from the token endpoint defined in OAuth 2.0 [RFC6749], Section 5.1. This endpoint DOES NOT accept refresh tokens for validation. | 
 **scope** | **String**| An optional, space separated list of required scopes. If the access token was not granted one of the scopes, the result of active will be false. | 

### Return type

[**::models::OAuth2TokenIntrospection**](oAuth2TokenIntrospection.md)

### Authorization

[basic](../README.md#basic), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_o_auth2_clients**
> Vec<::models::OAuth2Client> list_o_auth2_clients(ctx, optional)
List OAuth 2.0 Clients

This endpoint lists all clients in the database, and never returns client secrets.  OAuth 2.0 clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities. To manage ORY Hydra, you will need an OAuth 2.0 Client as well. Make sure that this endpoint is well protected and only callable by first-party components.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:clients\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i64**| The maximum amount of policies returned. | 
 **offset** | **i64**| The offset from where to start looking. | 

### Return type

[**Vec<::models::OAuth2Client>**](oAuth2Client.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **oauth_auth**
> oauth_auth()
The OAuth 2.0 authorize endpoint

This endpoint is not documented here because you should never use your own implementation to perform OAuth2 flows. OAuth2 is a very popular protocol and a library for your programming language will exists.  To learn more about this flow please refer to the specification: https://tools.ietf.org/html/rfc6749

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **oauth_token**
> ::models::OauthTokenResponse oauth_token(ctx, ctx, )
The OAuth 2.0 token endpoint

This endpoint is not documented here because you should never use your own implementation to perform OAuth2 flows. OAuth2 is a very popular protocol and a library for your programming language will exists.  To learn more about this flow please refer to the specification: https://tools.ietf.org/html/rfc6749

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::OauthTokenResponse**](oauthTokenResponse.md)

### Authorization

[basic](../README.md#basic), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reject_o_auth2_consent_request**
> reject_o_auth2_consent_request(ctx, id, consent_request_rejection)
Reject a consent request

Call this endpoint to reject a consent request. This usually happens when a user denies access rights to an application.   The consent request id is usually transmitted via the URL query `consent`. For example: `http://consent-app.mydomain.com/?consent=1234abcd`   The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:oauth2:consent:requests:<request-id>\"], \"actions\": [\"reject\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
  **consent_request_rejection** | [**ConsentRequestRejection**](ConsentRequestRejection.md)|  | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **revoke_o_auth2_token**
> revoke_o_auth2_token(ctx, token)
Revoke OAuth2 tokens

Revoking a token (both access and refresh) means that the tokens will be invalid. A revoked access token can no longer be used to make access requests, and a revoked refresh token can no longer be used to refresh an access token. Revoking a refresh token also invalidates the access token that was created with it.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basic](../README.md#basic)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_o_auth2_client**
> ::models::OAuth2Client update_o_auth2_client(ctx, id, o_auth2_client)
Update an OAuth 2.0 Client

Update an existing OAuth 2.0 Client. If you pass `client_secret` the secret will be updated and returned via the API. This is the only time you will be able to retrieve the client secret, so write it down and keep it safe.  OAuth 2.0 clients are used to perform OAuth 2.0 and OpenID Connect flows. Usually, OAuth 2.0 clients are generated for applications which want to consume your OAuth 2.0 or OpenID Connect capabilities. To manage ORY Hydra, you will need an OAuth 2.0 Client as well. Make sure that this endpoint is well protected and only callable by first-party components.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:clients\"], \"actions\": [\"update\"], \"effect\": \"allow\" } ```  Additionally, the context key \"owner\" is set to the owner of the client, allowing policies such as:  ``` { \"resources\": [\"rn:hydra:clients\"], \"actions\": [\"update\"], \"effect\": \"allow\", \"conditions\": { \"owner\": { \"type\": \"EqualsSubjectCondition\" } } } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**|  | 
  **o_auth2_client** | [**OAuth2Client**](OAuth2Client.md)|  | 

### Return type

[**::models::OAuth2Client**](oAuth2Client.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **userinfo**
> ::models::UserinfoResponse userinfo(ctx, )
OpenID Connect Userinfo

This endpoint returns the payload of the ID Token, including the idTokenExtra values, of the provided OAuth 2.0 access token. The endpoint implements http://openid.net/specs/openid-connect-core-1_0.html#UserInfo .

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::UserinfoResponse**](userinfoResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **well_known**
> ::models::JsonWebKeySet well_known(ctx, )
Get Well-Known JSON Web Keys

Returns metadata for discovering important JSON Web Keys. Currently, this endpoint returns the public key for verifying OpenID Connect ID Tokens.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:hydra.openid.id-token:public\"], \"actions\": [\"GET\"], \"effect\": \"allow\" } ```

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::JsonWebKeySet**](jsonWebKeySet.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

